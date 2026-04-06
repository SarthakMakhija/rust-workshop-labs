use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

// ❓ Our Cache is generic and safe, but only in a single thread.
// 🤔 Questions: 
// - What happens if we try to share a '&mut Cache' across threads?
// - Hint: Search for the "Shared XOR Mutable" rule in Rust.
// - Why are we using 'RwLock' instead of a regular 'Mutex'?
struct Cache<K, V>
where
    K: Hash + Eq,
{
    // 💡 Interior Mutability: Turning a Shared Reference (&) 
    //   into a Mutable Reference (&mut) safely at runtime.
    entries: RwLock<HashMap<K, V>>
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Cache<K, V> {
        Self {
            entries: RwLock::new(HashMap::new())
        }
    }

    // ❓ Notice that 'put' currently takes '&self'.
    // 🤔 Question: Do we need &mut self? If yes, why? If not, why?
    fn put(&self, key: K, value: V) {
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    // ❓ The Lifetime Paradox.
    // 🤔 Questions: 
    // - Why does this 'get' return 'Option<V>' instead of 'Option<&V>'?
    // - What happens to the RwLockReadGuard when this function returns?
    // - If we returned a reference to the data (&V), would it still be 
    //   protected by the lock after the function ends?
    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // 🚀 The RAII Guard: Locking is automatic, unlocking is also automatic.
        // 🤔 Question: Why do we call .unwrap() on the lock? 
        // - Hint: What is a "Poisoned Lock"?
        // TODO: Implement lookup from the HashMap
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache: Cache<String, String> = Cache::new();
        let value = cache.get(&String::from("test"));

        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(value, "2026");
    }
}