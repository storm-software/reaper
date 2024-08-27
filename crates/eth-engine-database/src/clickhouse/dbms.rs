use db_interfaces::{clickhouse_dbms, remote_clickhouse_table};
use reaper_eth_engine_types::{
  db::{
    address_to_protocol_info::ProtocolInfoClickhouse, block_analysis::BlockAnalysis,
    dex::DexQuotesWithBlockNumber, normalized_actions::TransactionRoot,
    token_info::TokenInfoWithAddress, DbDataWithRunId, RunId,
  },
  mev::*,
};

clickhouse_dbms!(
  ReaperEthEngineClickhouseTables,
  "eth_cluster0",
  [
    ReaperEthEngineDex_Price_Mapping,
    ReaperEthEngineBlock_Analysis,
    MevMev_Blocks,
    MevBundle_Header,
    MevSearcher_Tx,
    MevCex_Dex_Quotes,
    MevCex_Dex,
    MevLiquidations,
    MevJit_Sandwich,
    MevJit,
    MevSandwiches,
    MevAtomic_Arbs,
    ReaperEthEngineToken_Info,
    EthereumPools,
    ReaperEthEngineTree,
    ReaperEthEngineRun_Id
  ]
);

impl ReaperEthEngineClickhouseTables {
  pub const fn is_big(&self) -> bool {
    matches!(
      self,
      ReaperEthEngineClickhouseTables::ReaperEthEngineDex_Price_Mapping
        | ReaperEthEngineClickhouseTables::ReaperEthEngineTree
    )
  }
}

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Reaper, Dex_Price_Mapping],
  DexQuotesWithBlockNumber,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Reaper, Block_Analysis],
  DbDataWithRunId<BlockAnalysis>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Mev_Blocks],
  DbDataWithRunId<MevBlock>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Bundle_Header],
  DbDataWithRunId<BundleHeader>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Searcher_Tx],
  DbDataWithRunId<SearcherTx>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Cex_Dex],
  DbDataWithRunId<CexDex>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Cex_Dex_Quotes],
  DbDataWithRunId<CexDexQuote>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Liquidations],
  DbDataWithRunId<Liquidation>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Jit_Sandwich],
  DbDataWithRunId<JitLiquiditySandwich>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Jit],
  DbDataWithRunId<JitLiquidity>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Sandwiches],
  DbDataWithRunId<Sandwich>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Atomic_Arbs],
  DbDataWithRunId<AtomicArb>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Reaper, Token_Info],
  TokenInfoWithAddress,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Ethereum, Pools],
  ProtocolInfoClickhouse,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Reaper, Tree],
  DbDataWithRunId<TransactionRoot>,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Reaper, Run_Id],
  RunId,
  "crates/reaper-eth-engine-database/src/clickhouse/tables/"
);

pub struct ReaperEthEngineClickhouseData {
  pub data:         ReaperEthEngineClickhouseTableDataTypes,
  pub force_insert: bool,
}

macro_rules! db_types {
    ($(($db_type:ident, $db_table:ident, $t:tt)),*) => {
        db_types!(enum_s {}, $($db_type, $t,)*);

        paste::paste! {
            impl ReaperEthEngineClickhouseTableDataTypes {
                pub fn get_db_enum(&self) -> ReaperEthEngineClickhouseTables {
                    match self {
                        $(
                            ReaperEthEngineClickhouseTableDataTypes::$db_type(_) =>
                                ReaperEthEngineClickhouseTables::$db_table,
                        )*
                    }
                }
            }
        }

        $(
            db_types!($db_type, $t);

        )*
    };
    ($db_type:ident, true) => {
            impl From<($db_type, bool, u64)> for ReaperEthEngineClickhouseData {
                fn from(value: ($db_type, bool, u64)) ->ReaperEthEngineClickhouseData {
                    ReaperEthEngineClickhouseData {
                        data: ReaperEthEngineClickhouseTableDataTypes::$db_type(Box::new(
                                      DbDataWithRunId {
                                          table: value.0,
                                          run_id: value.2
                                      }
                                      )),
                        force_insert: value.1
                    }
                }
            }

    };
    ($db_type:ident, false) => {
        impl From<($db_type, bool)> for ReaperEthEngineClickhouseData {
            fn from(value: ($db_type, bool)) ->ReaperEthEngineClickhouseData {
                ReaperEthEngineClickhouseData {
                    data: ReaperEthEngineClickhouseTableDataTypes::$db_type(Box::new(value.0)),
                    force_insert: value.1
                }
            }
        }
    };
    (enum_s  {$($acc:tt)* }, $db_type:ident, true, $($tail:tt)*) => {
        db_types!(enum_s {
            $($acc)*
            $db_type(Box<DbDataWithRunId<$db_type>>),
        }, $($tail)*);
    };
    (enum_s {$($acc:tt)* }, $db_type:ident, false, $($tail:tt)*) => {
        db_types!(enum_s {
            $($acc)*
            $db_type(Box<$db_type>),
        }, $($tail)*);
    };
    (enum_s {$($acc:tt)*},$(,)*) => {
        #[derive(Debug, Clone, serde::Serialize)]
        #[serde(untagged)]
        #[allow(clippy::large_enum_variant)]
        pub enum ReaperEthEngineClickhouseTableDataTypes {
            $($acc)*
        }
    }
}

db_types!(
  (DexQuotesWithBlockNumber, ReaperEthEngineDex_Price_Mapping, false),
  (MevBlock, MevMev_Blocks, true),
  (BundleHeader, MevBundle_Header, true),
  (SearcherTx, MevSearcher_Tx, true),
  (CexDex, MevCex_Dex, true),
  (CexDexQuote, MevCex_Dex_Quotes, true),
  (Liquidation, MevLiquidations, true),
  (JitLiquiditySandwich, MevJit_Sandwich, true),
  (JitLiquidity, MevJit, true),
  (Sandwich, MevSandwiches, true),
  (AtomicArb, MevAtomic_Arbs, true),
  (TokenInfoWithAddress, ReaperEthEngineToken_Info, false),
  (ProtocolInfoClickhouse, EthereumPools, false),
  (TransactionRoot, ReaperEthEngineTree, true),
  (BlockAnalysis, ReaperEthEngineBlock_Analysis, true),
  (RunId, ReaperEthEngineRun_Id, false)
);
