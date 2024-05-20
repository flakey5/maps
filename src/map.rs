pub trait Map<K, V: AsRef<V>> {
    fn contains(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, val: V);
    fn delete(&mut self, key: &K) -> Option<V>;
}

