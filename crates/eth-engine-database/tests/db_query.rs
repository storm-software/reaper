use std::time::Duration;

use futures::{stream::FuturesUnordered, StreamExt};
use reaper_eth_engine_database::{clickhouse::Clickhouse, libmdbx::cex_utils::CexRangeOrArbitrary};

mod shared;
use rand::Rng;
use shared::fetch_test_data;
use tokio::time::sleep;

#[reaper_eth_engine_macros::test]
async fn test_query_retry() {
  let client = Clickhouse::new_default(None).await;

  let range = CexRangeOrArbitrary::Range(19000000, 19001000);

  let mut futs = FuturesUnordered::new();

  for _ in 0..30 {
    futs.push(async {
      let mut rng = rand::thread_rng();
      sleep(Duration::from_millis(rng.gen_range(10..100))).await;
      fetch_test_data(&client, range).await
    });
  }

  while let Some(result) = futs.next().await {
    assert!(result.is_ok());
  }
}
