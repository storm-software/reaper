use std::{
  pin::Pin,
  task::{Context, Poll},
  time::{Duration, Instant},
};

use db_interfaces::{
  clickhouse::{client::ClickhouseClient, config::ClickhouseConfig},
  Database,
};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use reaper_eth_engine_types::{
  db_write_trigger::HeartRateMonitor, FastHashMap, UnboundedYapperReceiver,
};
use reth_tasks::shutdown::GracefulShutdown;
use tokio::task::JoinError;

use crate::clickhouse::dbms::*;

type InsertFut = Pin<Box<dyn Future<Output = Result<eyre::Result<()>, JoinError>> + Send>>;

pub struct ClickhouseBuffered {
  client:            ClickhouseClient<ReaperEthEngineClickhouseTables>,
  rx:                UnboundedYapperReceiver<Vec<ReaperEthEngineClickhouseData>>,
  value_map:
    FastHashMap<ReaperEthEngineClickhouseTables, Vec<ReaperEthEngineClickhouseTableDataTypes>>,
  buffer_size_small: usize,
  buffer_size_big:   usize,
  futs:              FuturesUnordered<InsertFut>,
  /// if none, will always write to db. if some. will only start writing if
  heart_rate:        Option<HeartRateMonitor>,
  skip:              bool,
}

impl ClickhouseBuffered {
  pub fn new(
    rx: UnboundedYapperReceiver<Vec<ReaperEthEngineClickhouseData>>,
    config: ClickhouseConfig,
    buffer_size_small: usize,
    buffer_size_big: usize,
    heart_rate: Option<HeartRateMonitor>,
  ) -> Self {
    Self {
      client: config.build(),
      rx,
      value_map: FastHashMap::default(),
      buffer_size_small,
      buffer_size_big,
      skip: heart_rate.is_some(),
      heart_rate,
      futs: FuturesUnordered::default(),
    }
  }

  fn handle_incoming(&mut self, value: Vec<ReaperEthEngineClickhouseData>) {
    let enum_kind = value.first().as_ref().unwrap().data.get_db_enum();
    let mut force_insert = false;

    let entry = self.value_map.entry(enum_kind.clone()).or_default();

    entry.extend(value.into_iter().map(|value| {
      force_insert |= value.force_insert;
      value.data
    }));

    let size = if enum_kind.is_big() { self.buffer_size_big } else { self.buffer_size_small };

    if entry.len() >= size || force_insert {
      let client = self.client.clone();
      self.futs.push(Box::pin(tokio::spawn(Self::insert(
        client,
        std::mem::take(entry),
        enum_kind,
      ))));
    }
  }

  async fn insert(
    client: ClickhouseClient<ReaperEthEngineClickhouseTables>,
    data: Vec<ReaperEthEngineClickhouseTableDataTypes>,
    table: ReaperEthEngineClickhouseTables,
  ) -> eyre::Result<()> {
    macro_rules! inserts {
            ($(($table_id:ident, $inner:ident)),+) => {
                match table {
                    $(
                        ReaperEthEngineClickhouseTables::$table_id => {
                            let insert_data = data
                                .into_iter()
                                .filter_map(|d| match d {
                                    ReaperEthEngineClickhouseTableDataTypes::$inner(inner_data) => {
                                        Some(*inner_data)
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>();

                            if insert_data.is_empty() {
                                panic!("you did this wrong idiot");
                            }
                            client
                                .insert_many::<$table_id>(&insert_data)
                                .await?
                        },
                    )+
                }
            };
        }

    inserts!(
      (MevBundle_Header, BundleHeader),
      (MevMev_Blocks, MevBlock),
      (MevCex_Dex_Quotes, CexDexQuote),
      (MevCex_Dex, CexDex),
      (MevSearcher_Tx, SearcherTx),
      (MevJit, JitLiquidity),
      (MevJit_Sandwich, JitLiquiditySandwich),
      (MevSandwiches, Sandwich),
      (MevAtomic_Arbs, AtomicArb),
      (MevLiquidations, Liquidation),
      (ReaperEthEngineDex_Price_Mapping, DexQuotesWithBlockNumber),
      (ReaperEthEngineToken_Info, TokenInfoWithAddress),
      (EthereumPools, ProtocolInfoClickhouse),
      (ReaperEthEngineTree, TransactionRoot),
      (ReaperEthEngineBlock_Analysis, BlockAnalysis),
      (ReaperEthEngineRun_Id, RunId)
    );

    Ok(())
  }

  /// Done like this to avoid runtime load and ensure we always are sending
  pub fn run(self, shutdown: GracefulShutdown) {
    std::thread::spawn(move || {
      tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
          self.run_to_completion(shutdown).await;
        });
    });
  }

  pub async fn run_to_completion(mut self, shutdown: GracefulShutdown) {
    let mut pinned = std::pin::pin!(self);
    let mut shutdown_g = None;
    tokio::select! {
        _ = &mut pinned => {}
        i = shutdown => {
            shutdown_g = Some(i);
        }
    };
    pinned.shutdown().await;

    // we do this so doesn't get instant dropped by compiler
    tracing::trace!(was_shutdown = shutdown_g.is_some());
    drop(shutdown_g);
  }

  pub async fn shutdown(&mut self) {
    tracing::info!("starting shutdown process clickhouse writer");

    let mut last_message = Instant::now();
    // if we go 1s without a message, we assume shutdown was complete
    while last_message.elapsed() < Duration::from_secs(1) {
      let mut message = false;
      while let Ok(value) = self.rx.try_recv() {
        if value.is_empty() {
          continue
        }

        message = true;

        let enum_kind = value.first().as_ref().unwrap().data.get_db_enum();
        let entry = self.value_map.entry(enum_kind.clone()).or_default();
        entry.extend(value.into_iter().map(|v| v.data));
      }

      for (enum_kind, entry) in &mut self.value_map {
        if entry.is_empty() {
          continue
        }

        self.futs.push(Box::pin(tokio::spawn(Self::insert(
          self.client.clone(),
          std::mem::take(entry),
          enum_kind.clone(),
        ))));
      }
      // inserts take some time so we update last message here
      if message {
        last_message = Instant::now();
      }
    }

    while (self.futs.next().await).is_some() {}
  }
}

impl Future for ClickhouseBuffered {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let this = self.get_mut();
    let mut work = 128;

    loop {
      if let Some(hr) = this.heart_rate.as_mut() {
        match hr.poll_next_unpin(cx) {
          Poll::Ready(Some(val)) => {
            this.skip = val;
          }
          Poll::Ready(None) => return Poll::Ready(()),
          Poll::Pending => {}
        }
      }

      let mut cnt = 500;
      while let Poll::Ready(val) = this.rx.poll_recv(cx) {
        match val {
          Some(val) if !this.skip => {
            if !val.is_empty() {
              this.handle_incoming(val)
            }
          }
          Some(_) => {}
          None => return Poll::Ready(()),
        }

        cnt -= 1;
        if cnt == 0 {
          break
        }
      }

      while let Poll::Ready(Some(val)) = this.futs.poll_next_unpin(cx) {
        if let Err(e) = val {
          tracing::error!(target: "reaper-eth-engine", "error writing to clickhouse {:?}", e);
        }
      }

      work -= 1;
      if work == 0 {
        cx.waker().wake_by_ref();
        return Poll::Pending
      }
    }
  }
}