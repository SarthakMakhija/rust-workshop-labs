use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard};
use std::time::{Duration, Instant};

// ❓ Our Cache has been great at storing data, but it has a "Memory Bloat" problem.
// 🤔 Questions: 
// - What happens to keys that are no longer needed? Do they stay in memory forever?
// - How can we track when a specific piece of data should "expire"?
struct Entry<V> {
    value: V,
    // 💡 The Metadata Layer: Every value now carries its own "best-before" date.
    expires_at: Instant,
}

// ❓ A Shard now manages two distinct pieces of state.
// 🤔 Questions: 
// - Why do we store the expiration in BOTH the Entry and a separate 'ttl_list'?
// - Hint: How would a background thread find expired keys without scanning 
//   the entire HashMap (which could have millions of entries)?
struct Shard<K, V>
where
    K: Hash + Eq + Clone,
{
    entries: RwLock<HashMap<K, Entry<V>>>,
    // 💡 The TTL List: A specialized structure for order-based cleanup.
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

    // 🤔 Questions:
    // - Why do we have 2 locks?
    // - Can we reduce the scope of these locks in put?
    // - What is "Lock Contention"? How do these small scopes help?
    // - Why do we need to 'key.clone()' when inserting into the entries map?
    fn put(&self, key: K, value: V, ttl: Duration) {
        // TODO: Implement sharded insertion with TTL.
        // 1. Calculate the 'expires_at' Instant.
        // 2. Wrap the value in an 'Entry'.
        // 3. Store in 'entries' (in a scoped write lock).
        // 4. Store in 'ttl_list' (in a scoped write lock).
        unimplemented!()
    }

    // ❓ Getting data involves more than just a lookup now.
    // 🤔 Questions: 
    // - What is "Lazy Expiration"? 
    // - If we find an expired entity, why do we return 'None' even if
    //   it's still physically present in the HashMap?
    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement sharded lookup with lazy expiration.
        // 1. Acquire the read lock.
        // 2. Look up the entry.
        // 3. Check for expiration.
        // 4. Return the Zero-Copy Ref if valid.
        unimplemented!()
    }

    // 🚀 The Cleanup Phase.
    // ❓ We need to remove thousands of expired keys without blocking the entire system.
    // 🤔 Questions: 
    // - Why do we perform the cleanup in TWO distinct phases?
    // - Phase 1: Retaining non-expired keys and collecting expired ones.
    // - Phase 2: Removing from the entries map.
    // - What would happen if we held BOTH locks simultaneously for the entire duration?
    fn cleanup(&self) {
        // TODO: Implement the two-phase cleanup.
        // 1. Acquire write lock on 'ttl_list'.
        // 2. Use 'retain' to remove expired items and collect their keys.
        // 3. Drop the 'ttl_list' lock as soon as possible!
        // 4. Acquire write lock on 'entries' only if there are keys to remove.
        unimplemented!()
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
        unsafe { &*self.value }
    }
}

#[cfg(test)]
mod shard_tests {
    use std::thread;
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

    #[test]
    fn get_expired_key() {
        let shard = Shard::new();
        shard.put(String::from("rustconf"), String::from("2026"), Duration::from_millis(2));

        thread::sleep(Duration::from_millis(5));

        let value = shard.get("rustconf");
        assert!(value.is_none());
    }
}
