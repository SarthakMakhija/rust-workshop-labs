use std::collections::HashMap;

// ❓ We've replaced raw Strings with these NewTypes.
// 🤔 Questions: 
// - This is called the "NewType Pattern". Why do we do this?
// - Does wrapping a String in a struct make the program slower?
// - Hint: Look up "Zero Cost Abstractions" in Rust.
#[derive(Hash, PartialEq, Eq)]
struct CacheKey(String);

#[derive(Debug, PartialEq)]
struct CacheValue(String);

// ❓ Our Cache now uses specific types.
// 🤔 Question: 
// - If we accidentally swap 'key' and 'value' in the parameters, 
//   what will happen now compared to Stage 1?
struct Cache {
    entries: HashMap<CacheKey, CacheValue>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            entries: HashMap::new(),
        }
    }

    fn put(&mut self, key: CacheKey, value: CacheValue) {
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    // 🤔 Question: Why do we need the 'Hash' and 'Eq' traits on CacheKey?
    // - Hint: How does a HashMap know where to store a value?
    // - How does it know if two keys are exactly the same?
    fn get(&self, key: &CacheKey) -> Option<&CacheValue> {
        // TODO: Implement lookup in the HashMap
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache = Cache::new();
        let value = cache.get(&CacheKey("hello".to_string()));

        assert_eq!(None, value);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(CacheKey(String::from("rustconf")), CacheValue(String::from("2026")));

        // ❓ Unambiguous signatures. 
        // 🤔 Question: Try swapping the key/value here. What happens?
        let value = cache.get(&CacheKey(String::from("rustconf")));
        assert_eq!(Some(&CacheValue(String::from("2026"))), value);
    }

    #[test]
    fn attempt_get_a_non_existent_key() {
        let mut cache = Cache::new();
        cache.put(CacheKey(String::from("rustconf")), CacheValue(String::from("2026")));

        let value = cache.get(&CacheKey(String::from("non-existent")));
        assert_eq!(None, value);
    }
}