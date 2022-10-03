use std::fmt::Debug;
use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const DEFAULT_CAPACITY: usize = 8;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

type LinkedList<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct HashMap<K, V> {
    table: Vec<LinkedList<K, V>>,
    size: usize,
    load_factor: f32,
    capacity: usize,
    threshold: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq, //+ Debug + Clone,
{
    pub fn new() -> Self {
        let threshold = (DEFAULT_CAPACITY as f32 * DEFAULT_LOAD_FACTOR) as usize;
        Self {
            table: Vec::new(),
            size: 0,
            capacity: DEFAULT_CAPACITY,
            load_factor: DEFAULT_LOAD_FACTOR,
            threshold,
        }
    }

    pub fn put(&mut self, new_key: K, new_value: V) -> Option<V> {
        if self.table.len() == 0
        /* || self.size > self.threshold  */
        {
            self.resize();
        }

        let index = self.index_for(&new_key);
        // check if there is the same key already
        match self.column_iter_mut(index).find(|(k, _v)| *k == new_key) {
            // if so replace its value & return the old one
            Some(pair) => {
                let old_value = mem::replace(&mut pair.1, new_value);
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

    pub fn into_iter(mut self) -> IntoIter<K, V> {
        let mut index = 0;
        let mut next = None;

        loop {
            if index == self.table.len() {
                break;
            }
            if let Some(_) = self.table[index].take().map(|n| {
                next = Some(n);
            }) {
                break;
            }
            index += 1;
        }

        IntoIter {
            map: self,
            next,
            index,
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        let mut iter = Iter {
            next: None,
            index: 0,
            map: self,
        };

        loop {
            if iter.index == self.table.len() {
                break;
            }

            if let Some(_) = &self.table[iter.index].as_ref().map(|n| {
                iter.next = Some(&n);
            }) {
                iter.index += 1;
                break;
            }

            iter.index += 1;
        }
        iter
    }

    fn column_iter_mut(&mut self, index: usize) -> ColumnIterMut<K, V> {
        ColumnIterMut {
            next: self.table[index].as_deref_mut(),
        }
    }

    fn resize(&mut self) {
        let capacity;
        match self.table.len() {
            0 => capacity = DEFAULT_CAPACITY,
            n => capacity = n * 2,
        }
        self.table = (0..capacity).map(|_| None).collect();
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

pub struct Iter<'a, K, V> {
    next: Option<&'a Node<K, V>>,
    map: &'a HashMap<K, V>,
    index: usize,
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            if node.next.is_none() {
                loop {
                    if self.index == self.map.table.len() {
                        self.next = None;
                        break;
                    };

                    if let Some(n) = &self.map.table[self.index] {
                        self.next = Some(&n);
                        self.index += 1;
                        break;
                    }
                    self.index += 1;
                }
            } else {
                self.next = node.next.as_deref();
            }
            &node.pair
        })
    }
}

pub struct IntoIter<K, V> {
    map: HashMap<K, V>,
    next: LinkedList<K, V>,
    index: usize,
}
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            if node.next.is_none() {
                loop {
                    if self.index == self.map.table.len() {
                        break;
                    }
                    if let Some(_) = self.map.table[self.index].take().map(|n| {
                        self.next = Some(n);
                    }) {
                        break;
                    }
                    self.index += 1;
                }
            } else {
                self.next = node.next;
            }
            node.pair
        })
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
    use crate::DEFAULT_CAPACITY;

    use super::HashMap;

    #[test]
    fn basic() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.put(1, 555);
        assert_eq!(map.size(), 1);
        //default capacity
        assert_eq!(map.table.len(), DEFAULT_CAPACITY);
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

    #[test]
    fn iter() {
        let mut map = HashMap::new();

        map.put("a", 17);
        map.put("b", 78);
        map.put("c", 777);

        let mut pairs_count = 0;

        for (k, v) in map.iter() {
            match k {
                &"a" => assert_eq!(*v, 17),
                &"b" => assert_eq!(*v, 78),
                &"c" => assert_eq!(*v, 777),
                _ => unreachable!(),
            }
            pairs_count += 1;
        }

        assert_eq!(pairs_count, 3);
    }

    #[test]
    fn into_iter() {
        let mut map = HashMap::new();
        map.put("one", 1);
        map.put("two", 2);
        map.put("three", 3);

        let res: Vec<(&str, i32)> = map.into_iter().collect();

        assert_eq!(res, vec![("three", 3), ("two", 2), ("one", 1)]);
    }
}
