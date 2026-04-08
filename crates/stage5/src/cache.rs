use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;

// ❓ We are introducing "Interior Mutability" using RefCell.
// 🤔 Questions: 
// - Why did we wrap V in a RefCell inside the Entry?
// - How does this allow us to change data even when we only have 
//   an immutable reference (&self) to the Cache?
#[derive(PartialEq, Debug)]
struct Entry<V> {
    value: RefCell<V>,
}

impl<V> Entry<V> {
    fn new(value: V) -> Entry<V> {
        Self {
            value: RefCell::new(value),
        }
    }

    fn get(&self) -> Ref<'_, V> {
        unimplemented!()
    }

    fn get_mut(&self) -> RefMut<'_, V> {
        // 💡 This method does NOT require &mut self! 
        // 🤔 Is it possible to have two callers get a mutable reference 
        //   to the same Entry at the same time? What happens if they try?
        unimplemented!()
    }
}

struct Cache<K, V>
where
    K: Hash + Eq,
{
    entries: HashMap<K, Entry<V>>,
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
        // TODO: Implement insertion into the HashMap
        unimplemented!()
    }

    fn get<Q>(&self, key: &Q) -> Option<&Entry<V>>
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
        let value = cache.get(&String::from("test"));

        assert_eq!(value, None);
    }

    #[test]
    fn get_existing_key() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value.get(), "2026");
    }

    #[test]
    fn mutate_value() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));

        {
            let value = cache.get("rustconf").unwrap();
            let mut value = value.get_mut();
            *value = String::from("2027");
        }
        // 💡 Crucial Scope! 
        // 🤔 Questions:
        // - Why did we wrap the logic above in a block { }? 
        // - What would happen if we removed the block and tried to call 
        //   value.get() on the final line? (Hint: already mutably borrowed).

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value.get(), "2027");
    }

    #[test]
    fn mutate_values() {
        let mut cache = Cache::new();
        cache.put(String::from("rustconf"), String::from("2026"));
        cache.put(String::from("dbconf"), String::from("2026"));

        // 💡 Granular Mutability: The Breakthrough of Stage 5.
        // 🤔 Questions:
        // - In Stage 4, why was it impossible to have two mutable references like this?
        // - Why does it work now?
        // - Notice that "rustconf" and "dbconf" have their OWN RefCells. 
        //   Locking one room does not stop you from entering another!
        {
            let rustconf_value = cache.get("rustconf").unwrap();
            let mut rustconf_value = rustconf_value.get_mut();
            *rustconf_value = String::from("2027");

            let dbconf_value = cache.get("dbconf").unwrap();
            let mut dbconf_value = dbconf_value.get_mut();
            *dbconf_value = String::from("2027");
        } // 💡 All RefMut's are dropped here!

        let value = cache.get("rustconf").unwrap();
        assert_eq!(*value.get(), "2027");

        let value = cache.get("dbconf").unwrap();
        assert_eq!(*value.get(), "2027");
    }
}
