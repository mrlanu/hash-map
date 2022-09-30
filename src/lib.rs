use std::fmt::Debug;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const DEFAULT_CAPACITY: usize = 4;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

type LinkedList<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct HashMap<K, V>
where
    K: Debug + Eq + PartialEq,
    V: Debug,
{
    table: Vec<LinkedList<K, V>>,
    size: usize,
    load_factor: f32,
    capacity: usize,
    threshold: u32,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq + Debug + Clone,
    V: Debug + Clone,
{
    pub fn new() -> Self {
        let threshold = (DEFAULT_CAPACITY as f32 * DEFAULT_LOAD_FACTOR) as u32;
        Self {
            table: (0..DEFAULT_CAPACITY)
                .collect::<Vec<usize>>()
                .into_iter()
                .map(|_| None)
                .collect(),
            size: 0,
            capacity: DEFAULT_CAPACITY,
            load_factor: DEFAULT_LOAD_FACTOR,
            threshold,
        }
    }
    pub fn put(&mut self, new_key: K, new_value: V) -> Option<V> {
        let index = self.index_for(&new_key);
        println!("Index: {}", index);
        // check if there is the same key already
        match self.column_iter_mut(index).find(|(k, v)| *k == new_key) {
            // if so replace its value & return the old one
            Some(pair) => {
                let old_value = pair.1.clone();
                (*pair).1 = new_value;
                return Some(old_value);
            }
            // if none just push new pair
            None => {
                self.push_pair(index, new_key, new_value);
            }
        }
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn push_pair(&mut self, index: usize, key: K, value: V) {
        let mut new_node = Box::new(Node::new(key, value));
        new_node.next = self.table[index].take();
        self.table[index] = Some(new_node);
        self.size += 1;
    }

    fn index_for(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.table.len()
    }

    fn column_iter_mut(&mut self, index: usize) -> ColumnIterMut<K, V> {
        ColumnIterMut {
            next: self.table[index].as_deref_mut(),
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    pair: (K, V),
    next: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            pair: (key, value),
            next: None,
        }
    }
}

pub struct ColumnIterMut<'a, K, V> {
    next: Option<&'a mut Node<K, V>>,
}
impl<'a, K, V> Iterator for ColumnIterMut<'a, K, V> {
    type Item = &'a mut (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.pair
        })
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn basic() {
        let map: HashMap<i32, i32> = HashMap::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn insert() {
        let mut map: HashMap<String, String> = HashMap::new();
        assert_eq!(map.size(), 0);

        map.put("key_1".to_string(), "value_1".to_string());
        map.put("key_2".to_string(), "value_2".to_string());
        assert_eq!(map.size(), 2);

        let old_value = map.put("key_1".to_string(), "value_2".to_string());
        assert_eq!(old_value, Some("value_1".to_string()));
        assert_eq!(map.size(), 2);
    }
}
