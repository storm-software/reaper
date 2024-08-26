CREATE TABLE reaper.token_info ON CLUSTER eth_cluster0
(
    `address` String,
    `symbol` String,
    `decimals` UInt8
)
ENGINE = ReplicatedReplacingMergeTree('/clickhouse/eth_cluster0/tables/all/reaper_eth_engine/token_info', '{replica}')
PRIMARY KEY `address`
ORDER BY `address`



