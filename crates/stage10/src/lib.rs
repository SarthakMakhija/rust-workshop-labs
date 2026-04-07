use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, RwLock};

struct Cache<K, V>
where
    K: Hash + Eq,
{
    shards: Vec<RwLock<HashMap<K, Arc<V>>>>,
    num_shards: usize,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    fn new(num_shards: usize) -> Arc<Self> {
        Arc::new(Self {
            shards: (0..num_shards)
                .map(|_| RwLock::new(HashMap::new()))
                .collect::<Vec<_>>(),
            num_shards,
        })
    }

    fn put(&self, key: K, value: V) {
        // TODO: Implement thread-safe insertion.
        let shard_index = self.shard_index(&key);
        self.shards[shard_index].write().unwrap().insert(key, Arc::new(value));
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement thread-safe lookup.
        let shard_index = self.shard_index(key);
        self.shards[shard_index].read().unwrap().get(key).cloned()
    }

    fn shard_index<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        hasher.finish() as usize % self.num_shards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Arc<Cache<String, String>> = Cache::new(8);
        let value = cache.get("test");

        assert!(value.is_none());
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
    fn put() {
        let cache = Cache::new(16);

        let thread_handles = (1..=10)
            .map(|counter| {
                let cache_clone = cache.clone();

                // 💡 The 'move' keyword transfers ownership of the clone into the thread.
                thread::spawn(move || {
                    cache_clone.put(counter, counter.to_string());
                })
            })
            .collect::<Vec<_>>();

        for handle in thread_handles {
            handle.join().unwrap();
        }

        let thread_handles = (1..=10)
            .map(|counter| {
                let cache_clone = cache.clone();
                thread::spawn(move || {
                    let value = cache_clone.get(&counter).unwrap();
                    assert_eq!(*value, counter.to_string());
                })
            }).collect::<Vec<_>>();

        for handle in thread_handles {
            handle.join().unwrap();
        }
    }
}
