SELECT
    (block_number, tx_idx),
    tx_idx,
    quote
FROM reaper.dex_price_mapping
WHERE block_number >= ? AND block_number < ?
