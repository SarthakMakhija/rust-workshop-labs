use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

// ❓ Stage 6 was safe, but we paid a "Cloning Tax".
// 🤔 Questions: 
// - If 'V' is an 8MB image, what happens in Stage 6 every time we call 'get'?
// - How does shared ownership solve this without data races?
struct Cache<K, V>
where
    K: Hash + Eq,
{
    // 💡 Arc (Atomic Reference Counting): A smart pointer that 
    //   allows multiple owners on the heap.
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

    // ❓ We still take V by value.
    // 🤔 Question: What happens to 'value' when we wrap it in Arc::new()?
    // 🤔 Question: Who owns V?
    fn put(&self, key: K, value: V) {
        // TODO: Implement insertion into the HashMap, wrapping value in an Arc
        unimplemented!()
    }

    // ❓ Notice the return type: 'Option<Arc<V>>'.
    // 🤔 Questions: 
    // - Does .cloned() here copy the underlying data 'V'?
    // - Why did we remove the 'V: Clone' bound from the impl block?
    // - What is the cost of cloning an Arc handle?
    fn get<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
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
