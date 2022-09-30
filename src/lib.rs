use std::fmt::Debug;

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
    K: Eq + PartialEq + Debug + Clone,
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

    pub fn size(&self) -> usize {
        self.size
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

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn basic() {
        let map: HashMap<i32, i32> = HashMap::new();
        assert_eq!(map.size(), 4);
    }
}
