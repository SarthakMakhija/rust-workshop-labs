use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

// ❓ We've made our Cache generic using placeholders K and V.
// 🤔 Questions: 
// - What is "Monomorphization"? 
// - Does the use of generics like <K, V> make our program slower at runtime compared to Stage 1?
// - Why do we need the 'where' clause here?
//   (Hint: Try removing the Hash + Eq bounds and see what happens).
struct Cache<K, V>
where
    K: Hash + Eq,
{
    entries: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Cache<K, V> {
        Self {
            entries: HashMap::new(),
        }
    }

    fn put(&mut self, key: K, value: V) {
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    // ❓ This method takes a reference to K. 
    // 🤔 Question: If K is a String, we still have to pass &String. 
    //   Does this solve the "Allocation Problem" from Stage 1?
    fn get(&self, key: &K) -> Option<&V> {
        // TODO: Implement lookup from the HashMap
        unimplemented!()
    }

    // 🚀 The solution to our "Allocation Problem": The Borrow Trait.
    // 🤔 Questions: 
    // - How does a HashMap<String, V> allow a &str lookup? (Hint: K: Borrow<Q>).
    // - What is ?Sized? Why is it necessary for types like 'str'?
    // - Why do we need Hash + Eq bounds on Q as well?
    fn get_improved<Q>(&self, key: &Q) -> Option<&V>
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
        // 🤔 Question: Why is it required to specify <String, String>?
        let value = cache.get(&String::from("test"));

        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        // ❓ This is standard lookup. 
        // 🤔 Question: What is the cost of calling String::from("rustconf") just to perform a lookup?
        let value = cache.get(&String::from("rustconf")).unwrap();
        assert_eq!(value, "2026");
    }

    #[test]
    fn get_existing_key_improved() {
        let mut cache = Cache::new();
        // 💡 This cache handles Strings but allows lookups using &str.
        cache.put(String::from("rustconf"), String::from("2026"));

        // 🚀 No allocation! 
        // 🤔 Question: Why does cache.get_improved("rustconf") work? 
        // - Hint: Borrow trait and Deref coercion converge.
        let value = cache.get_improved("rustconf").unwrap();
        assert_eq!(value, "2026");
    }

    #[test]
    fn get_existing_key_for_a_non_string_cache() {
        // 🚀 The ultimate flexibility.
        let mut cache = Cache::new();
        cache.put(1, "Sarthak".to_string());
        
        // 🤔 Question: Why can we pass an integer (1) to the same 'put' method?
        // - What concrete code is generated during Monomorphization?
        assert_eq!(cache.get_improved(&1).unwrap(), "Sarthak");
    }
}