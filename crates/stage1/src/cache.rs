use std::collections::HashMap;

// ❓ Our first iteration uses String for both keys and values.
// 🤔 Questions:
// - Is String allocated on heap?
// - What does it look like on stack?
// - Where does the literal "rust" go?
// - Who "owns" the data stored in the cache?
struct Cache {
    entries: HashMap<String, String>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            entries: HashMap::new(),
        }
    }

    // 💡 This method modifies the internal state of the cache.
    // 🤔 Questions:
    // - Why is the receiver `&mut self` instead of `&self`?
    // - What would happen if we tried to insert using an immutable reference?
    // - What is the meaning of &mut self. It is a mutable reference, brilliant, but what does that mean?
    fn put(&mut self, key: String, value: String) {
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    // ❓ This method currently expects `&String`.
    // 🤔 Questions:
    // - What happens if you try to call get("key")? Try it and observe the compiler error.
    // - Hint: This is known as "The Allocation Problem". We are forced to 
    //   allocate a full String on the heap just to perform a lookup!
    //
    // 🤔 Questions:
    // - Why return `Option<&String>` instead of `Option<String>`?
    // - What would happen to the data inside the `HashMap` if we returned `Option<String>`?
    // - Do we want to "take" the value or just "look" at it?
    //
    // 💡 Challenge: Can you change the signature to take `&str` instead?
    fn get(&self, key: &String) -> Option<&String> {
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
        // ❓ Note how we are forced to create a full `String` just to query!
        // `&String::from("hello")` allocates memory on the heap.
        // 🤔 Question: What if we could just pass `&"hello"`?
        let value = cache.get(&String::from("hello"));

        // 🤔 Question: What is the ! after assert_eq?
        // - Is it a function or something else?
        // - Hint: Look at unimplemented! above. Do they share a common trait?
        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        // 🤔 Question: Why is cache declared "mut"?
        // - What would happen if we removed "mut" and then called put?
        cache.put(String::from("rustconf"), String::from("2026"));

        // ❓ Another allocation just for a lookup...
        let value = cache.get(&String::from("rustconf"));
        assert_eq!(value, Some(&String::from("2026")));
    }

    #[test]
    fn attempt_get_a_non_existent_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get(&String::from("non-existent"));
        // 🤔 Question: What is this Option type? How does it help us avoid common bugs in other languages?
        assert_eq!(value, None);
    }
}