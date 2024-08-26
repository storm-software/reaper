CREATE TABLE reaper_eth_engine.run_id ON CLUSTER eth_cluster0
(
    `run_id` UInt64,
    `last_updated` UInt64 DEFAULT now()
)
ENGINE = ReplicatedReplacingMergeTree('/clickhouse/eth_cluster0/tables/all/reaper_eth_engine/run_id', '{replica}', last_updated)
ORDER BY run_id
SETTINGS index_granularity = 8192
