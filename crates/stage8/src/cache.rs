use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard};

// ❓ We've reached the final optimization: Zero-Allocation access.
// 🤔 Questions: 
// - If we return a reference directly from a guarded HashMap, why does it fail?
// - How can we return a reference while keeping the lock alive?
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

    // 🚀 The Goal: Return a reference to the data without cloning.
    // 🤔 Questions: 
    // - What is the '?' operator doing here? (Hint: Early Return).
    // - Why do we return a 'Ref' struct instead of '&V'?
    // - Trace the lifetime: If 'Ref' is dropped, what happens to the lock?
    fn get<Q>(&self, key: &Q) -> Option<Ref<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement zero-copy lookup.
        // 1. Acquire the read lock.
        // 2. Get the reference from the map (use the ? operator).
        // 3. Convert the reference to a raw pointer (*const V).
        // 4. Return the Ref struct.
        unimplemented!()
    }
}

// ❓ This is a "Custom Ref" pattern.
// 🤔 Questions: 
// - Why does this struct have a lifetime parameter 'a?
// - What is the purpose of storing the 'guard' inside this struct?
// - What is '*const V'? Why can't we just store '&V'? 
//   (Hint: Search for "Self-Referential Structs" in Rust).
struct Ref<'a, K, V>
where
    K: Hash + Eq,
{
    // 💡 The Anchor: This keeps the lock alive as long as 'Ref' exists.
    guard: RwLockReadGuard<'a, HashMap<K, V>>,
    // 💡 The Target: A raw pointer into the data owned by the guard.
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

// ❓ Making our custom struct behave like the underlying data.
// 🤔 Questions: 
// - Why is the deref implementation marked as 'unsafe'?
// - Is deref unsafe or the block unsafe?
// - What invariant are we manually upholding to make this safe?
// - Compare this to Stage 7: Which one holds the lock for longer?
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
