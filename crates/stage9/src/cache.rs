use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

struct Cache<K, V>
where
    K: Hash + Eq,
{
    entries: RwLock<HashMap<K, Arc<V>>>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Arc<Cache<K, V>> {
        Arc::new(Self {
            entries: RwLock::new(HashMap::new()),
        })
    }

    fn put(&self, key: K, value: V) {
        self.entries.write().unwrap().insert(key, Arc::new(value));
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.entries.read().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Arc<Cache<String, String>> = Cache::new();
        let value = cache.get("test");

        assert!(value.is_none());
    }

    #[test]
    fn get_existing_key() {
        let cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2026");
    }
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::thread;

    #[test]
    fn put() {
        let cache = Cache::new();
        let thread_handles = (1..=10)
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
