use std::thread;
use std::time::Duration;
use tinycache::cache::Cache;
use tinycache::error::CacheError;

// 🚀 The Integration Layer: Real-World Verification.
// ❓ Testing from the Outside In.
// 🤔 Questions: 
// - Rust compiles every file in the 'tests/' directory as a separate crate. 
// - Why does this matter for our testing strategy? (Hint: Can we access 'pub(crate)' items?)
// - This is commony called "Black-Box" testing. How does this differ from the 
//   "White-Box" tests we wrote inside 'cache.rs'?
// - Which type of test gives you more confidence that your library won't 
//   break for other developers?

#[test]
fn get_value_for_existing_key() {
    let cache = Cache::new(8);
    cache
        .put("key".to_string(), "val".to_string(), Duration::from_secs(1))
        .unwrap();

    assert!(cache.get("key").unwrap().is_some());
}

// 💡 Real-World Scenario: Ensuring data lives exactly as long as promised.
// 💡 Real-World Scenario: Verifying that data adheres to its Time-To-Live.
// 🤔 Questions: 
// - In this test, we use 'thread::sleep'. In a high-performance system, is this 
//   a "good" way to test? 
// - What are the trade-offs between "Real Clock" testing and "Mock Clock" testing?
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

// 💡 Robustness: Ensuring the library signals its state to the caller.
// 🤔 Questions: 
// - Why is 'matches!(..., Err(CacheError::ShuttingDown))' better than 
//   just checking if 'is_err()' is true?
// - How does this help a developer build a more resilient application?
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

// 💡 Thread Safety: All clones must observe the same global state.
// 🤔 Questions: 
// - We shut down the master 'cache', but we check the 'cache_clone'.
// - How does the 'AtomicBool' inside CacheInner ensure all these handles 
//   see the shutdown signal at the exact same time (conceptually)?
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