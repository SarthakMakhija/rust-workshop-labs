use std::collections::HashMap;

// ❓ We've replaced raw Strings with these NewTypes.
// 🤔 Questions:
// - This is called the "NewType Pattern". Why do we do this?
// - Does wrapping a String in a struct make the program slower?
// - Hint: Look up "Zero Cost Abstractions" in Rust.
#[derive(Hash, PartialEq, Eq)]
struct CacheKey(String);

impl From<&str> for CacheKey {
    // 🤔 Questions: 
    // - Why do we use .to_string() here? 
    fn from(value: &str) -> Self {
        CacheKey(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
struct CacheValue(String);

impl From<&str> for CacheValue {
    fn from(value: &str) -> Self {
        CacheValue(value.to_string())
    }
}

// ❓ Custom equality for cleaner assertions.
// 🤔 Questions:
// - Why do we implement PartialEq<str>?
// - How does it help us in our unit-tests?
impl PartialEq<str> for CacheValue {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

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
        self.entries.insert(key, value);
    }

    // 🤔 Question: Why do we need the 'Hash' and 'Eq' traits on CacheKey?
    // - Hint: How does a HashMap know where to store a value?
    // - How does it know if two keys are exactly the same?
    fn get(&self, key: &CacheKey) -> Option<&CacheValue> {
        self.entries.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache = Cache::new();
        // 🤔 Question: This is verbose, Create CacheKey -> Pass String and finally
        // pass a reference of CacheKey. Can something be done?
        let value = cache.get(&CacheKey(String::from("hello")));

        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(CacheKey(String::from("rustconf")), CacheValue(String::from("2026")));

        // ❓ Unambiguous signatures.
        // 🤔 Question: Try swapping the key/value here. What happens?
        let value = cache.get(&CacheKey(String::from("rustconf")));
        assert_eq!(value, Some(&CacheValue(String::from("2026"))));
    }

    #[test]
    fn get_existing_key_ergonomic() {
        let mut cache = Cache::new();

        // 💡 Ergonomic! No more manual wrapping or .to_string() calls.
        // 🤔 Question: Why does .into() work for both CacheKey and CacheValue here?
        cache.put("rustconf".into(), "2026".into());

        // 🤔 Question: Is using .unwrap() here safe? Why might we accept it in
        //   a test but avoid it in production code?
        let value = cache.get(&"rustconf".into()).unwrap();

        // 💡 Clean assertion! 
        // 🤔 Question: How can we compare a `CacheValue` (a struct) with a literal `"2026"` (&str)?
        //   (Hint: Look at our PartialEq implementation above).
        assert_eq!(value, "2026");
    }
}
