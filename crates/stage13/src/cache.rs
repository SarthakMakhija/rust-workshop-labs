use crate::shard::{Ref, Shard};
use std::borrow::Borrow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// 💡 The Handle-Body Pattern: Decoupling Lifecycle from Data.
// ❓ We've split the cache into 'Cache' (the handle) and 'CacheInner' (the data).
// 🤔 Why this design? 
// - 'Cache' is a lightweight handle that can be cloned and moved into threads.
// - 'CacheInner' is the heavy, shared data sitting behind an 'Arc'.
// - This allows us to have 'Master' vs 'Worker' semantics. For example:
//   Only the Master Handle (the one with 'cleaner') can shut down the cache,
//   even though all handles share the same shards!
pub struct Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    inner: Arc<CacheInner<K, V>>,
    cleaner: Option<JoinHandle<()>>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    pub fn new(num_shards: usize) -> Self {
        let inner = CacheInner::new(num_shards);
        let handle = Self::spawn_expired_keys_cleaner(inner.clone());
        Self {
            inner,
            cleaner: Some(handle),
        }
    }

    pub fn put(&self, key: K, value: V, ttl: Duration) {
        self.inner.put(key, value, ttl);
    }

    pub fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key)
    }

    pub fn shutdown(mut self) {
        // ❓ We take 'mut self' by value.
        // 🤔 Questions: 
        // - After this method returns, can the caller still use the 'cache' variable?
        // - How does Rust's ownership system turn a logic bug (using a closed cache) 
        //   into a compiler error?
        self.inner.mark_shutting_down();

        // 💡 The Join: Closing the loop on background work.
        // TODO: wait for the cleaner thread to finish.
        unimplemented!()
    }

    fn spawn_expired_keys_cleaner(inner: Arc<CacheInner<K, V>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut shard_index = 0;
            loop {
                // ❓ The Cleaner holds an Arc<CacheInner>.
                // 🤔 Questions:
                // - What would happen to the memory of CacheInner if this loop never ended?
                // - Why do we need a signal to stop this thread manually?
                // TODO: return if the cache is shutting down.

                let shard = &inner.shards[shard_index];
                shard.cleanup();

                shard_index = (shard_index + 1) % inner.shards.len();
                thread::sleep(Duration::from_millis(500));
            }
        })
    }
}

impl<K, V> Clone for Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        // ❓ 'Cache' is a handle, but 'CacheInner' is the data.
        // 🤔 Questions: 
        // - Why is 'cleaner' set to 'None' in the clone? 
        // - What would happen if multiple clones tried to '.join()' the same JoinHandle?
        // - Does cloning the handle clone the Shards? (Hint: check Arc).
        // TODO: clone the Cache.
        unimplemented!()
    }
}

struct CacheInner<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    shards: Vec<Shard<K, V>>,
    num_shards: usize,
    shutting_down: AtomicBool,
}

impl<K, V> CacheInner<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    fn new(num_shards: usize) -> Arc<Self> {
        let shards = (0..num_shards).map(|_| Shard::new()).collect();
        Arc::new(Self {
            shards,
            num_shards,
            shutting_down: AtomicBool::new(false),
        })
    }

    fn put(&self, key: K, value: V, ttl: Duration) {
        // 🚀 The Hardware Coordination.
        // ❓ We've reached the point where every operation pays a "Tax".
        // 🤔 Questions: 
        // - Why do we check 'shutting_down' before every single insertion?
        // - Why can't the "Type-State" pattern on the handle globally eliminate 
        //   this check when using Arc? (Hint: Think about other threads).
        // TODO: ensure that the cache is not shutting down.
        self.shards[self.shard_index(&key)].put(key, value, ttl);
    }

    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // 💡 Runtime Safety Check.
        // TODO: ensure that the cache is not shutting down.
        self.shards[self.shard_index(&key)].get(key)
    }

    fn mark_shutting_down(&self) {
        self.shutting_down.store(true, Ordering::Release);
    }

    fn shard_index<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_shards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_when_cache_is_empty() {
        let cache = Cache::<String, String>::new(8);
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn get_returns_some_when_key_exists() {
        let cache = Cache::new(8);
        cache.put("key".to_string(), "val".to_string(), Duration::from_secs(1));

        assert!(cache.get("key").is_some());
    }

    #[test]
    fn get_returns_correct_value() {
        let cache = Cache::new(8);
        cache.put("key".to_string(), "val".to_string(), Duration::from_secs(1));

        let value = cache.get("key").unwrap();
        assert_eq!(*value, "val");
    }

    #[test]
    fn get_returns_none_after_expiration() {
        let cache = Cache::new(8);
        cache.put("key".to_string(), "val".to_string(), Duration::from_millis(2));

        thread::sleep(Duration::from_millis(5));

        assert!(cache.get("key").is_none());
    }

    #[test]
    fn put_is_silently_ignored_after_shutdown() {
        let cache = Cache::new(8);
        let worker_handle = cache.clone();
        
        cache.shutdown();
        worker_handle.put("key".to_string(), "val".to_string(), Duration::from_secs(1));
        
        // No assertion needed as silent failure is the goal, but we verify get follows
        assert!(worker_handle.get("key").is_none());
    }

    #[test]
    fn get_returns_none_immediately_after_shutdown() {
        let cache = Cache::new(8);
        let cache_clone = cache.clone();

        cache.put("key".to_string(), "val".to_string(), Duration::from_secs(1));
        cache.shutdown();
        
        assert!(cache_clone.get("key").is_none());
    }
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;

    #[test]
    fn multiple_threads_can_put_and_get_concurrently() {
        let cache = Cache::new(16);
        let mut handles = vec![];

        for counter in 0..10 {
            let c = cache.clone();
            handles.push(thread::spawn(move || {
                for j in 0..100 {
                    let key = format!("{}-{}", counter, j);
                    c.put(key.clone(), "val".to_string(), Duration::from_secs(10));
                    assert_eq!(*c.get(&key).unwrap(), "val");
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        cache.shutdown();
    }

    #[test]
    fn shutdown_signals_all_threads_to_stop_safely() {
        let cache: Cache<String, String> = Cache::new(8);
        let cache_clone = cache.clone();
        
        cache.shutdown();
        
        // This should return None immediately even if data was there
        assert!(cache_clone.get("any").is_none());
    }
}