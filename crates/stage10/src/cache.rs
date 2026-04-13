use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard};

// ❓ Even with Arc and zero-copy access, we have a "Global Lock".
// 🤔 Questions:
// - If 100 threads try to write to 100 different keys, why do they still block?
// - What is "Lock Contention"?
// - How can we "partition" our data so that writers mostly don't interfere?
struct Cache<K, V>
where
    K: Hash + Eq,
{
    // 💡 Sharding: Instead of one big lock, we have N independent locks.
    shards: Vec<RwLock<HashMap<K, V>>>,
    num_shards: usize,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    fn new(num_shards: usize) -> Arc<Self> {
        // TODO: Implement sharded initialization.
        // 1. Create a Vec of 'num_shards' independent RwLocks.
        // 2. Wrap it all in an Arc.
        unimplemented!()
    }

    fn put(&self, key: K, value: V) {
        // TODO: Implement sharded insertion.
        // 1. Find the correct shard index for the key. (Take a look at DefaultHasher).
        // 2. Acquire the write lock on ONLY that shard.
        // 3. Insert the value.
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
    //   (💡Hint: Bitwise AND vs Modulo operator).
    fn shard_index<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_shards
    }
}

struct Ref<'a, K, V>
where
    K: Hash + Eq,
{
    guard: RwLockReadGuard<'a, HashMap<K, V>>,
    value: *const V,
}

impl<'a, K, V> Ref<'a, K, V>
where
    K: Hash + Eq,
{
    fn new(guard: RwLockReadGuard<'a, HashMap<K, V>>, value: *const V) -> Ref<'a, K, V> {
        Self { guard, value }
    }
}

impl<'a, K, V> Deref for Ref<'a, K, V>
where
    K: Hash + Eq,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        // ❓ Dereferencing the raw pointer.
        // 🤔 Questions:
        // - Is the 'deref' method itself unsafe, or just the block inside it?
        // - Why is this "Safe Unsafe"? What invariant are we manually
        //   upholding here that the compiler can't see?
        unsafe { &*self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Arc<Cache<String, String>> = Cache::new(8);
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn get_existing_key() {
        let cache = Cache::new(8);
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2026");
    }
}

#[cfg(test)]
mod concurrency_test {
    use super::*;
    use std::thread;

    #[test]
    fn sharded_put() {
        // 💡 16 independent shards -> many threads can write in parallel
        //   without ever touching the same lock!
        let cache = Cache::new(16);

        let thread_handles = (1..=100)
            .map(|counter| {
                let cache_clone = cache.clone();
                thread::spawn(move || {
                    cache_clone.put(counter, counter.to_string());
                })
            })
            .collect::<Vec<_>>();

        for handle in thread_handles {
            handle.join().unwrap();
        }

        // 🚀 Scaling test: All 100 entries should be there.
        for counter in 1..=100 {
            let value = cache.get(&counter).unwrap();
            assert_eq!(*value, counter.to_string());
        }
    }
}
