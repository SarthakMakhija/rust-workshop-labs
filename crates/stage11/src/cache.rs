use crate::shard::{Ref, Shard};
use std::borrow::Borrow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;

struct Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    shards: Vec<Shard<K, V>>,
    num_shards: usize,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn new(num_shards: usize) -> Arc<Self> {
        // TODO: Implement sharded initialization.
        unimplemented!()
    }

    fn put(&self, key: K, value: V, ttl: Duration) {
        // TODO: Implement sharded insertion.
        unimplemented!()
    }

    // 🚀 Combining Zero-Copy access with Sharding.
    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement sharded zero-copy lookup.
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
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

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
