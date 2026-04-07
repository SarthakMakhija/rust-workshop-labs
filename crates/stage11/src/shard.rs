use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard};
use std::time::{Duration, Instant};

struct Entry<V> {
    value: V,
    expires_at: Instant,
}

struct Shard<K, V>
where
    K: Hash + Eq + Clone,
{
    entries: RwLock<HashMap<K, Entry<V>>>,
    ttl_list: RwLock<Vec<(K, Instant)>>,
}

impl<K, V> Shard<K, V>
where
    K: Hash + Eq + Clone,
{
    fn new() -> Shard<K, V> {
        Self {
            entries: RwLock::new(HashMap::new()),
            ttl_list: RwLock::new(Vec::new()),
        }
    }

    fn put(&self, key: K, value: V, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        {
            let mut guard = self.entries.write().unwrap();
            guard.insert(key.clone(), Entry { value, expires_at });
        }
        {
            let mut guard = self.ttl_list.write().unwrap();
            guard.push((key, Instant::now()));
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let guard = self.entries.read().unwrap();
        let entry = guard.get(key)?;
        if Instant::now() > entry.expires_at {
            return None;
        }

        let ptr = &entry.value as *const V;
        Some(Ref { guard, value: ptr })
    }
}

struct Ref<'a, K, V>
where
    K: Hash + Eq,
{
    guard: RwLockReadGuard<'a, HashMap<K, Entry<V>>>,
    value: *const V,
}

impl<'a, K, V> Ref<'a, K, V>
where
    K: Hash + Eq,
{
    fn new(guard: RwLockReadGuard<'a, HashMap<K, Entry<V>>>, value: *const V) -> Ref<'a, K, V> {
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
mod shard_tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_shard() {
        let shard: Shard<String, String> = Shard::new();
        assert!(shard.get("test").is_none());
    }

    #[test]
    fn get_existing_key() {
        let shard = Shard::new();
        shard.put(String::from("rustconf"), String::from("2026"), Duration::from_secs(1));

        let value = shard.get("rustconf").unwrap();
        assert_eq!(*value, "2026");
    }
}
