mod linked_list;

use crate::linked_list::{Iter as IterLL, LinkedList};
use std::fmt::Debug;
use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const DEFAULT_CAPACITY: usize = 8;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

#[derive(Debug)]
pub struct HashMap<K, V> {
    pub table: Vec<LinkedList<(K, V)>>,
    // amount of pairs
    size: usize,
    _load_factor: f32,
    capacity: usize,
    // hash_map will double its capacity when this variable will be reached
    threshold: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + PartialEq, //+ Debug + Clone,
    V: Debug,
{
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            size: 0,
            capacity: DEFAULT_CAPACITY,
            _load_factor: DEFAULT_LOAD_FACTOR,
            threshold: (DEFAULT_CAPACITY as f32 * DEFAULT_LOAD_FACTOR) as usize,
        }
    }

    pub fn put(&mut self, new_key: K, new_value: V) -> Option<V> {
        if self.table.len() == 0 || self.size >= self.threshold {
            self.resize();
        }

        let index = self.index_for(&new_key);

        // check if map contains particular key
        match self.table[index].iter_mut().find(|(k, _v)| *k == new_key) {
            //if so replace the old value
            Some(pair) => {
                let ov = mem::replace(&mut pair.1, new_value);
                Some(ov)
            }
            //if none, push new pair to that existing list
            None => {
                self.table[index].push((new_key, new_value));
                self.size += 1;
                None
            }
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        let mut res = None;
        let index = self.index_for(&key);
        let mut new_list = LinkedList::new();
        let temp = mem::replace(&mut self.table[index], LinkedList::new());
        temp.into_iter().for_each(|(k, v)| {
            if k == key {
                self.size -= 1;
                res = Some(v);
            } else {
                new_list.push((k, v));
            }
        });
        if new_list.size() > 0 {
            self.table[index] = new_list;
        }
        res
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> Iter<K, V> {
        let mut iter = None;
        let mut index = 0;

        // search a first linked_list in the table which has any nodes
        loop {
            if index == self.table.len() {
                break;
            }
            match &self.table[index].size() {
                0 => {
                    index += 1;
                }
                // when got one take a reference on its iter
                _ => {
                    iter = Some(self.table[index].iter());
                    index += 1;
                    break;
                }
            }
        }

        Iter {
            index,
            table: &self.table,
            iter,
        }
    }

    fn index_for(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.table.len()
    }

    fn resize(&mut self) {
        match self.table.len() {
            // default resizing
            0 => {
                self.table = (0..DEFAULT_CAPACITY).map(|_| LinkedList::new()).collect();
            }
            // when this resize method is called after the threshold is reached
            n => {
                self.threshold = ((n * 2) as f32 * DEFAULT_LOAD_FACTOR) as usize;
                self.capacity *= 2;

                // replace the old table with new doubled one
                let mut temp = mem::replace(
                    &mut self.table,
                    (0..n * 2).map(|_| LinkedList::new()).collect(),
                );
                // reinsert the old table's values in the new one
                for i in 0..temp.len() {
                    let t = mem::replace(&mut temp[i], LinkedList::new());
                    t.into_iter().for_each(|pair| {
                        // minus 1 because actually it's not new pair
                        self.put(pair.0, pair.1);
                        self.size -= 1;
                    });
                }
            }
        }
    }
}

pub struct Iter<'a, K, V> {
    index: usize,
    table: &'a Vec<LinkedList<(K, V)>>,
    iter: Option<IterLL<'a, (K, V)>>,
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        // have to check if there is some iter
        // because the method iter() could be called on the empty map
        if let Some(iter) = &mut self.iter {
            // check whether this iter has a next node or not
            match iter.next() {
                // if so, return the reference on its contain
                Some(pair) => Some(pair),
                // if not, search a next list in the hash_map's table
                None => loop {
                    // break this loop on the end of Vec (table)
                    if self.index == self.table.len() {
                        break None;
                    }
                    match &self.table[self.index].size() {
                        // if the list size is 0 that means that it has no any next node
                        // just increment index of table
                        0 => {
                            self.index += 1;
                        }
                        _ => {
                            // if there is some size replace self.iter to its iter next value
                            // and return the reference to its contained value
                            self.iter = Some(self.table[self.index].iter());
                            self.index += 1;
                            break self.iter.as_mut().unwrap().next();
                        }
                    }
                },
            }
        } else {
            None
        }
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
    fn put() {
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
    fn get() {
        let mut map = HashMap::new();

        map.put("key_1".to_string(), "value_1".to_string());
        assert_eq!(map.size(), 1);

        let v = map.get("key_1".to_string());
        let n = map.get("empty".to_string());
        assert_eq!(v, Some("value_1".to_string()));
        assert_eq!(n, None);
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn resize() {
        let mut map = HashMap::new();
        map.put(1, 1);
        assert_eq!(map.size(), 1);
        assert_eq!(map.table.len(), DEFAULT_CAPACITY);
        map.get(1);
        assert_eq!(map.size(), 0);
        assert_eq!(map.table.len(), DEFAULT_CAPACITY);
        for i in 0..7 {
            map.put(i, i);
        }
        assert_eq!(map.size(), 7);
        assert_eq!(map.table.len(), DEFAULT_CAPACITY * 2);
        for i in 7..16 {
            map.put(i, i);
        }
        assert_eq!(map.size(), 16);
        assert_eq!(map.table.len(), DEFAULT_CAPACITY * 4);
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();

        let mut pairs_count = 0;

        for (k, v) in map.iter() {
            pairs_count += 1;
        }

        assert_eq!(pairs_count, 0);

        map.put("a", 17);
        map.put("b", 78);
        map.put("c", 777);

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

    // #[test]
    // fn into_iter() {
    //     let mut map = HashMap::new();
    //     map.put("one", 1);
    //     map.put("two", 2);
    //     map.put("three", 3);
    //
    //     let res: Vec<(&str, i32)> = map.into_iter().collect();
    //
    //     assert_eq!(res, vec![("three", 3), ("two", 2), ("one", 1)]);
    // }
}
