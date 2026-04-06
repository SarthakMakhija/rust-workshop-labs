use std::collections::HashMap;

struct Cache {
    entries: HashMap<String, String>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            entries: HashMap::new(),
        }
    }

    fn put(&mut self, key: String, value: String) {
        //TODO: fill in the answer
        unimplemented!()
    }

    fn get(&self, key: &String) -> Option<&String> {
        //TODO: fill in the answer
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_get_a_key_from_empty_cache() {
        let cache = Cache::new();
        let value = cache.get(&String::from("hello"));
        assert_eq!(None, value);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get(&String::from("rustconf"));
        assert_eq!(Some(&String::from("2026")), value);
    }

    #[test]
    fn attempt_get_a_non_existent_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get(&String::from("non-existent"));
        assert_eq!(None, value);
    }
}