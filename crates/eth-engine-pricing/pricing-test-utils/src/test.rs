use std::sync::{atomic::AtomicBool, Arc};

use alloy_primitives::{Address, TxHash};
use reaper_eth_engine_classifier::Classifier;
use reaper_eth_engine_core::test_utils::*;
use reaper_eth_engine_pricing::{types::DexPriceMsg, ReaperEthEngineBatchPricer, GraphManager};
use reaper_eth_engine_types::{
    normalized_actions::Action, traits::TracingProvider, tree::BlockTree, ReaperEthEngineTaskManager,
    FastHashMap, UnboundedYapperReceiver,
};
use thiserror::Error;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

type PricingResult<T> = Result<T, PricingTestError>;

pub struct PricingTestUtils {
    tracer:        TraceLoader,
    quote_address: Address,
}

impl PricingTestUtils {
    pub async fn new(quote_address: Address) -> Self {
        let tracer = TraceLoader::new().await;
        Self { tracer, quote_address }
    }

    async fn init_dex_pricer(
        &self,
        block: u64,
        end_block: Option<u64>,
        rx: UnboundedReceiver<DexPriceMsg>,
    ) -> Result<ReaperEthEngineBatchPricer<Box<dyn TracingProvider>>, PricingTestError> {
        let pairs = self
            .tracer
            .libmdbx
            .protocols_created_before(block)
            .map_err(|_| PricingTestError::LibmdbxError)?;

        let pair_graph = GraphManager::init_from_db_state(pairs, None);

        let created_pools = if let Some(end_block) = end_block {
            self.tracer
                .libmdbx
                .protocols_created_range(block + 1, end_block)
                .unwrap()
                .into_iter()
                .flat_map(|(_, pools)| {
                    pools
                        .into_iter()
                        .map(|(addr, protocol, pair)| (addr, (protocol, pair)))
                        .collect::<Vec<_>>()
                })
                .collect::<FastHashMap<_, _>>()
        } else {
            FastHashMap::default()
        };
        let ex = ReaperEthEngineTaskManager::current().executor();
        Ok(ReaperEthEngineBatchPricer::new(
            0,
            Arc::new(AtomicBool::new(false)),
            self.quote_address,
            pair_graph,
            UnboundedYapperReceiver::new(rx, 100_000, "test".into()),
            self.tracer.get_provider(),
            block,
            created_pools,
            Arc::new(AtomicBool::new(false)),
            None,
            ex,
        ))
    }

    /// traces the block and classifies it sending all messages to the batch
    /// pricer.
    pub async fn setup_dex_pricer_for_block(
        &self,
        block: u64,
    ) -> PricingResult<(ReaperEthEngineBatchPricer<Box<dyn TracingProvider>>, BlockTree<Action>)> {
        let BlockTracesWithHeaderAnd { traces, header, .. } =
            self.tracer.get_block_traces_with_header(block).await?;

        let (tx, rx) = unbounded_channel();
        let pricer = self.init_dex_pricer(block, None, rx).await?;

        let classifier = Classifier::new(self.tracer.libmdbx, tx, self.tracer.get_provider());
        let tree = classifier.build_block_tree(traces, header, true).await;
        Ok((pricer, tree))
    }

    pub async fn setup_dex_pricer_for_tx(
        &self,
        tx_hash: TxHash,
    ) -> Result<ReaperEthEngineBatchPricer<Box<dyn TracingProvider>>, PricingTestError> {
        let TxTracesWithHeaderAnd { trace, header, block, .. } =
            self.tracer.get_tx_trace_with_header(tx_hash).await?;
        let (tx, rx) = unbounded_channel();

        let classifier = Classifier::new(self.tracer.libmdbx, tx, self.tracer.get_provider());
        let pricer = self.init_dex_pricer(block, None, rx).await?;

        let _tree = classifier.build_block_tree(vec![trace], header, true).await;

        Ok(pricer)
    }
}

#[derive(Debug, Error)]
pub enum PricingTestError {
    #[error(transparent)]
    TraceError(#[from] TraceLoaderError),
    #[error("libmdbx error")]
    LibmdbxError,
}
