use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard};

struct Cache<K, V>
where
    K: Hash + Eq,
{
    entries: RwLock<HashMap<K, V>>,
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
        self.entries.write().unwrap().insert(key, value);
    }

    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement lookup from the HashMap
        let guard = self.entries.read().unwrap();
        let value = guard.get(key)?;

        let ptr = value as *const V;
        Some(Ref::new(guard, ptr))
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
        unsafe { &*self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Cache<String, String> = Cache::new();
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
