use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

// ❓ We've used 'thread::scope' before. Now we want true independence.
// 🤔 Questions: 
// - Why does 'std::thread::spawn' require us to move ownership?
// - What happens if a spawned thread outlives the 'main' function?
// - Why can't we just pass a '&Cache' to a spawned thread?
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
    // 💡 The pattern: Returning an Arc<Self> makes the cache a 
    //   "Shared Resource" by default.
    fn new() -> Arc<Cache<K, V>> {
        // TODO: Implement the Arc-wrapped initialization.
        unimplemented!()
    }

    fn put(&self, key: K, value: V) {
        // TODO: Implement thread-safe insertion.
        unimplemented!()
    }

    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement thread-safe lookup.
        unimplemented!()
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
        // 💡 We are using Arc::new() in Cache::new(), so we start with a shared handle to cache.
        let cache = Cache::new();

        // 🚀 Scaling to 10 independent threads.
        let thread_handles = (1..=10)
            .map(|counter| {
                // ❓ Each thread needs its own OWNED handle.
                // 🤔 Question: Why do we clone 'cache' inside the map instead of 
                //   moving the original 'cache' into the first thread?
                let cache_clone = cache.clone();
                
                // 💡 The 'move' keyword transfers ownership of the clone into the thread.
                thread::spawn(move || {
                    cache_clone.put(counter, counter.to_string());
                })
            })
            .collect::<Vec<_>>();

        // ❓ Synchronization point.
        // 🤔 Question: What happens if we try to read from the cache 
        //   BEFORE joining these threads?
        for handle in thread_handles {
            handle.join().unwrap();
        }

        // 🚀 Verify the results from another set of threads.
        let thread_handles = (1..=10)
            .map(|counter| {
                let cache_clone = cache.clone();
                thread::spawn(move || {
                    let value = cache_clone.get(&counter).unwrap();
                    assert_eq!(*value, counter.to_string());
                })
            }).collect::<Vec<_>>();

        // 💡 Wait for threads to finish.
        for handle in thread_handles {
            handle.join().unwrap();
        }
    }
}
