use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

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
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn put(&mut self, key: K, value: V) {
        // ❓ We are now modifying the state of the Cache.
        // 🤔 Questions: 
        // - Why must 'self' be '&mut self' here? 
        // - Is it possible to call 'put' if we only have a '&self' (immutable) reference?
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Implement lookup from the HashMap
        unimplemented!()
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // 💡 Exclusive Power: This method grants the caller the right to change a value.
        // 🤔 Questions:
        // - While the returned '&mut V' exists, what restrictions are placed on the 'cache'?
        // - Can we call 'get' (read) while we hold a 'get_mut' (write) reference?
        // TODO: Implement lookup from the HashMap, and return an optional mutable reference
        unimplemented!()
    }

    fn update<Q, F>(&mut self, key: &Q, block: F)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        F: FnOnce(&mut V),
    {
        // 💡 Inversion of Control: We pass the data to a closure instead of returning it.
        // 🤔 Questions:
        // - How does this pattern change the "lifetime" of the mutable borrow?
        // - Why is this often safer than returning a raw mutable reference?
        // TODO: Implement the update logic by fetching the mutable value 
        // and passing it to the 'block' closure.
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
        assert_eq!(*value, "2026");
    }


    #[test]
    fn get_mutable_value() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        // 💡 Scope Check: This starts a mutable borrow of 'cache'.
        // 🤔 While 'value' is alive and being used, can we call 'cache.get()'?
        let value = cache.get_mut("rustconf").unwrap();
        *value = String::from("2027");

        // 💡 The mutable borrow of 'cache' (via 'value') ends here because 
        // 'value' is no longer used afterward. This is why the 'get' call
        // below is allowed by the compiler!
        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2027");
    }

    #[test]
    fn get_mutable_values() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));
        cache.put(String::from("dbconf"), String::from("2026"));

        // 💡 Sequential Mutability:
        // This works because the lifetime of 'rustconf_value' ends 
        // before we request 'dbconf_value'.
        let rustconf_value = cache.get_mut("rustconf").unwrap();
        *rustconf_value = String::from("2027");

        let dbconf_value = cache.get_mut("dbconf").unwrap();
        *dbconf_value = String::from("2027");

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2027");

        let value = cache.get("dbconf").unwrap();
        assert_eq!(*value, "2027");
    }

    // 🚨 LEARNING MOMENT: Why won't this compile?
    // ❓ Try to uncomment the assertions below and run 'cargo test'.
    // 
    // 🤔 The Problem:
    // Rust's "Borrow Checker" enforces a strict rule: 
    // "You can have many immutable borrows (&T) OR exactly one mutable borrow (&mut T)."
    // 
    // When we call `cache.get_mut("rustconf")`, we create a mutable borrow of `cache`.
    // When we then call `cache.get_mut("dbconf")`, we try to create a SECOND mutable borrow.
    // 
    // Even though they point to different keys, the compiler only sees that 
    // both references depend on the SAME `entries` HashMap.
    #[test]
    fn get_mutable_values_non_compiling() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));
        cache.put(String::from("dbconf"), String::from("2026"));

        let _rustconf_value = cache.get_mut("rustconf").unwrap();
        // let _dbconf_value = cache.get_mut("dbconf").unwrap();

        // ❌ ERROR: Cannot borrow 'cache' as mutable more than once at a time.
        // assert_eq!(*_rustconf_value, "2026");
        // assert_eq!(*_dbconf_value, "2026");
    }

    #[test]
    fn update_value() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        cache.update("rustconf", |value| *value = String::from("2027"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2027");
    }

    #[test]
    fn update_values() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));
        cache.put(String::from("dbconf"), String::from("2026"));

        // 💡 Scope Management:
        // Notice how 'update' allows us to modify multiple keys sequentially 
        // without worrying about the borrow checker. Why? 
        // Because the mutable borrow is born and dies inside the 'update' call!
        cache.update("rustconf", |value| *value = String::from("2027"));
        cache.update("dbconf", |value| *value = String::from("2027"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value, "2027");

        let value = cache.get("dbconf").unwrap();
        assert_eq!(*value, "2027");
    }
}