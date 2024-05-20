// Reference: https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::map::Map;
use crate::tree_map::TreeMap;

const DEFAULT_SIZE: usize = 256;

pub struct HashMap<K: PartialEq + PartialOrd + Hash, V: AsRef<V>> {
    size: usize,
    data: Vec<KeyValue<K, V>>
}

impl<K: PartialEq + PartialOrd + Hash, V: AsRef<V>> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        let mut data = Vec::with_capacity(DEFAULT_SIZE);
        for _ in 0..DEFAULT_SIZE {
            data.push(KeyValue::new());
        }

        HashMap {
            size: 0,
            data
        }
    }
}

impl<K: PartialEq + PartialOrd + Hash, V: AsRef<V>> Map<K, V> for HashMap<K, V> {
    fn contains(&self, key: &K) -> bool {
        let pos = get_pos(key);
        self.data[pos].val.contains(key)
    } 

    fn get(&self, key: &K) -> Option<&V> {
        let pos = get_pos(key);
        self.data[pos].val.get(key)
    }

    fn set(&mut self, key: K, val: V) {
        let pos = get_pos(&key);
        self.data[pos].val.set(key, val);
    }

    fn delete(&mut self, key: &K) -> Option<V> {
        let pos = get_pos(&key);
        self.data[pos].val.delete(key)
    }
}

struct KeyValue<K: PartialEq + PartialOrd, V: AsRef<V>> {
   val: TreeMap<K, V>,
}

impl<K: PartialEq + PartialOrd, V: AsRef<V>> KeyValue<K, V> {
    pub fn new() -> KeyValue<K, V> {
        KeyValue {
            val: TreeMap::new(),
        }
    }
}

fn get_pos<K: Hash>(key: &K) -> usize {
    return (hash_key(key) as usize) % DEFAULT_SIZE;
}

fn hash_key<K: Hash>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    return hasher.finish();
}

