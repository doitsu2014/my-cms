use moka::future::Cache;
use std::time::Duration;

pub fn create_bucket_visibility_cache() -> Cache<String, bool> {
    Cache::builder()
        .max_capacity(256)
        .time_to_live(Duration::from_secs(300))
        .build()
}
