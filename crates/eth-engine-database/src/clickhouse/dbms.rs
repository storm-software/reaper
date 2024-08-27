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
    EngineDex_Price_Mapping,
    EngineBlock_Analysis,
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
    EngineToken_Info,
    EthereumPools,
    EngineTree,
    EngineRun_Id
  ]
);

impl ReaperEthEngineClickhouseTables {
  pub const fn is_big(&self) -> bool {
    matches!(
      self,
      ReaperEthEngineClickhouseTables::EngineDex_Price_Mapping
        | ReaperEthEngineClickhouseTables::EngineTree
    )
  }
}

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Engine, Dex_Price_Mapping],
  DexQuotesWithBlockNumber,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Engine, Block_Analysis],
  DbDataWithRunId<BlockAnalysis>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Mev_Blocks],
  DbDataWithRunId<MevBlock>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Bundle_Header],
  DbDataWithRunId<BundleHeader>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Searcher_Tx],
  DbDataWithRunId<SearcherTx>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Cex_Dex],
  DbDataWithRunId<CexDex>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Cex_Dex_Quotes],
  DbDataWithRunId<CexDexQuote>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Liquidations],
  DbDataWithRunId<Liquidation>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Jit_Sandwich],
  DbDataWithRunId<JitLiquiditySandwich>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Jit],
  DbDataWithRunId<JitLiquidity>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Sandwiches],
  DbDataWithRunId<Sandwich>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Mev, Atomic_Arbs],
  DbDataWithRunId<AtomicArb>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Engine, Token_Info],
  TokenInfoWithAddress,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Ethereum, Pools],
  ProtocolInfoClickhouse,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Engine, Tree],
  DbDataWithRunId<TransactionRoot>,
  "crates/eth-engine-database/src/clickhouse/tables/"
);

remote_clickhouse_table!(
  ReaperEthEngineClickhouseTables,
  [Engine, Run_Id],
  RunId,
  "crates/eth-engine-database/src/clickhouse/tables/"
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
  (DexQuotesWithBlockNumber, EngineDex_Price_Mapping, false),
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
  (TokenInfoWithAddress, EngineToken_Info, false),
  (ProtocolInfoClickhouse, EthereumPools, false),
  (TransactionRoot, EngineTree, true),
  (BlockAnalysis, EngineBlock_Analysis, true),
  (RunId, EngineRun_Id, false)
);
