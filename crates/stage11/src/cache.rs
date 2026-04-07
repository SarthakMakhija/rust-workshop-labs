use crate::shard::{Ref, Shard};
use std::borrow::Borrow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// 💡Create a type-alias for Shards.
// 🤔 Question: Why is the Vec<Shard<K, V>> wrapped in Arc?
type Shards<K: Hash + Eq + Clone, V> = Arc<Vec<Shard<K, V>>>;

struct Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    shards: Shards<K, V>,
    num_shards: usize,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    fn new(num_shards: usize) -> Arc<Self> {
        // TODO: Implement sharded initialization.
        // 1. Create the vector of Shards.
        // 2. Wrap the vector in an Arc to share it with the cleaner thread.
        // 3. Spawn the background cleaner.
        unimplemented!()
    }

    // ❓ Transparency: The Cache now requires a Duration.
    // 🤔 Question: Why is it okay for 'put' to remain &self even though 
    //   it eventually results in a mutation? (Hint: The Shards handle the locks).
    fn put(&self, key: K, value: V, ttl: Duration) {
        // TODO: Implement sharded insertion by delegating to the correct Shard.
        unimplemented!()
    }

    // 🚀 High-performance Lookup.
    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement lookup by delegating to the Shard.
        unimplemented!()
    }

    // ❓ Constant Routing.
    // 🤔 Questions:
    // - Why do we pay a "Hashing Tax" on every access?
    // - Why is the shard count often a "Power of 2" in production?
    //   (Hint: Bitwise AND vs Modulo operator).
    fn shard_index<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_shards
    }

    // ❓ The Background Cleaner.
    // 🤔 Questions:
    // - This thread uses an infinite 'loop' and 'thread::sleep'. 
    // - Is this thread "cooperative"?
    // - What happens to this thread when the 'Cache' is dropped? 
    // - Hint: Does it leak? How could we signal it to stop safely?
    fn spawn_expired_keys_cleaner(shards: Shards<K, V>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut shard_index = 0;
            loop {
                // 💡 Strategy: Clean one shard, sleep, then move to the next.
                // This spreads the "Cleanup Tax" over time.
                let shard = shards.get(shard_index).unwrap();
                shard.cleanup();

                shard_index = (shard_index + 1) % shards.len();
                thread::sleep(Duration::from_secs(1));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Arc<Cache<String, String>> = Cache::new(8);
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn get_existing_key() {
        let cache = Cache::new(8);
        cache.put(String::from("rustconf"), String::from("2026"), Duration::from_secs(1));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2026");
    }

    #[test]
    fn get_expired_key() {
        let cache = Cache::new(8);
        cache.put(String::from("rustconf"), String::from("2026"), Duration::from_millis(2));

        thread::sleep(Duration::from_millis(5));

        let value = cache.get("rustconf");
        assert!(value.is_none());
    }
}
