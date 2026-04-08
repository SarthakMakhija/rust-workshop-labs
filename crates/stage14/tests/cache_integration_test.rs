use std::thread;
use std::time::Duration;
use tinycache::cache::Cache;
use tinycache::error::CacheError;

#[test]
fn get_value_for_existing_key() {
    let cache = Cache::new(8);
    cache
        .put("key".to_string(), "val".to_string(), Duration::from_secs(1))
        .unwrap();

    assert!(cache.get("key").unwrap().is_some());
}

#[test]
fn attempt_to_get_a_value_after_expiration() {
    let cache = Cache::new(8);
    cache
        .put(
            "key".to_string(),
            "val".to_string(),
            Duration::from_millis(2),
        )
        .unwrap();

    thread::sleep(Duration::from_millis(5));

    assert!(cache.get("key").unwrap().is_none());
}

#[test]
fn put_returns_error_after_shutdown() {
    let cache = Cache::new(8);
    let worker_handle = cache.clone();

    cache.shutdown();
    let result =
        worker_handle.put("key".to_string(), "val".to_string(), Duration::from_secs(1));

    assert!(result.is_err());
    assert!(matches!(result, Err(CacheError::ShuttingDown)));
}

#[test]
fn get_returns_error_after_shutdown() {
    let cache = Cache::new(8);
    let cache_clone = cache.clone();

    cache
        .put("key".to_string(), "val".to_string(), Duration::from_secs(1))
        .unwrap();
    cache.shutdown();

    let result = cache_clone.get("key");
    assert!(result.is_err());
    assert!(matches!(result, Err(CacheError::ShuttingDown)));
}