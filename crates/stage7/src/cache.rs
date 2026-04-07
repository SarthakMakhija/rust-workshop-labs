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
    fn new() -> Cache<K, V> {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
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
        let cache: Cache<String, String> = Cache::new();
        let value = cache.get("test");

        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2026");
    }
}
