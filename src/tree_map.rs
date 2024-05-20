// Binary search tree
use crate::map::Map;

pub struct TreeMap<K: PartialEq + PartialOrd, V: AsRef<V>> {
    size: usize,
    head: Option<Box<KeyValue<K, V>>>,
}

impl<K: PartialEq + PartialOrd, V: AsRef<V>> TreeMap<K, V> {
    pub fn new() -> TreeMap<K, V> {
        TreeMap {
            size: 0,
            head: None
        }
    }

    fn get_impl(&self, key: &K) -> Option<&V> {
        if self.head.is_none() {
            return None
        }

        let mut cur = self.head.as_ref();
        while cur.is_some() {
            if *key == cur.unwrap().key {
                return Some(cur.unwrap().val.as_ref());
            } else if *key > cur.unwrap().key {
                cur = cur.unwrap().right.as_ref();
            } else {
                cur = cur.unwrap().left.as_ref();
            }
        }
        
        return None
    }
}

impl<K: PartialEq + PartialOrd, V: AsRef<V>> Map<K, V> for TreeMap<K, V> {
    fn contains(&self, key: &K) -> bool {
        self.get_impl(key).is_some()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get_impl(key)
    }

    fn set(&mut self, key: K, val: V) {
        if self.head.is_none() {
            self.head = Some(Box::new(KeyValue::new(key, val)));
            return;
        }

        let mut cur = self.head.as_mut().unwrap();
        loop {
            if key == cur.key {
                cur.val = val;
                break;
            } else if key > cur.key {
                if cur.right.is_some() {
                    cur = cur.right.as_mut().unwrap();
                } else {
                    cur.right = Some(Box::new(KeyValue::new(key, val)));
                    break;
                }
            } else {
                if cur.left.is_some() {
                    cur = cur.left.as_mut().unwrap();
                } else {
                    cur.left = Some(Box::new(KeyValue::new(key, val)));
                    break;
                }
            }
        }
    }

    fn delete(&mut self, key: &K) -> Option<V> {
        if self.head.is_none() {
            return None;
        }
        
        let mut cur = self.head.as_mut().unwrap();
        loop {
            if *key == cur.key {
                todo!();
            } else if *key > cur.key {
                if cur.right.is_some() {
                    cur = cur.right.as_mut().unwrap();
                } else {
                    return None;
                }
            } else {
                if cur.left.is_some() {
                    cur = cur.left.as_mut().unwrap();
                } else {
                    return None;
                }
            }
        }
    }
}

struct KeyValue<K: PartialEq + PartialOrd, V> {
    key: K,
    val: V,
    left: Option<Box<KeyValue<K, V>>>,
    right: Option<Box<KeyValue<K, V>>>,
}

impl<K: PartialEq + PartialOrd, V> KeyValue<K, V> {
    pub fn new(key: K, val: V) -> KeyValue<K, V> {
        KeyValue {
            key,
            val,
            left: None,
            right: None
        }
    }
}
