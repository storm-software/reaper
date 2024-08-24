SELECT
    block_number,
    traces
FROM reaper_eth_engine_api.tx_traces
WHERE block_number >= ? AND block_number < ?
