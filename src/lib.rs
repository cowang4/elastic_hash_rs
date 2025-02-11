use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distr::Distribution;


const PROBE_LIMIT: usize = 10;

#[derive(Debug)]
pub struct ElasticHashTable<K, V> {
    buckets: Vec<Option<(K, V)>>,
    size: usize,
    arrays: Vec<usize>,
}

impl<K: Eq + Hash + Clone, V: Clone> ElasticHashTable<K, V> {

    /// Create a new ElasticHashTable with the given power of 2 size.
    pub fn new(size: usize) -> Self {
        let buckets = vec![None; size];
        let arrays = Self::initialize_arrays(size);
        Self {
            buckets,
            size,
            arrays,
        }
    }

    /// Partition the table into arrays of exponentially decreasing size.
    fn initialize_arrays(size: usize) -> Vec<usize> {
        let mut arrays = Vec::new();
        let mut remaining_size = size;
        let max_iterations = (size as f64).log2().ceil() as usize;
        let mut i = 0;
        while remaining_size > 0 && i < max_iterations {
            let size = std::cmp::max(1, remaining_size / 2);
            arrays.push(size);
            remaining_size -= size;
            i += 1;
        }
        arrays
    }

     /// Generate a probe sequence (a vector of table indices) for a given key and array i.
     fn probe_sequence(&self, key: &K, i: usize) -> Vec<usize> {
        // Here, we compute a seed from the hash of the key plus the array index.
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let seed = hash.wrapping_add(i as u64);
        let mut rng = StdRng::seed_from_u64(seed);
        // Generate 10 random probe indices in the range 0 .. n.
        let between = rand::distr::Uniform::try_from(0..self.size).expect("Invalid probe range");
        (0..PROBE_LIMIT).map(|_| between.sample(&mut rng)).collect()
    }

    /// Insert a key into the table. Returns true if the key was inserted,
    /// or false if it was already present or if no slot was available.
    pub fn insert(&mut self, key: K, value: V) -> Result<(), Error> {
        for (i, _array_size) in self.arrays.iter().enumerate() {
            let probes = self.probe_sequence(&key, i);
            for probe in probes {
                match self.buckets[probe] {
                    None => {
                        self.buckets[probe] = Some((key, value));
                        return Ok(()); // successfully inserted
                    },
                    Some((ref existing_key, _)) if existing_key == &key => {
                        return Err(Error::KeyAlreadyInserted);
                    },
                    Some(_) => continue, // slot is occupied, try next probe
                }
            }
        }
        Err(Error::TableFull)
    }

    /// Search for a key in the table.
    /// Returns the value if found, or None if not found.
    pub fn get(&self, key: &K) -> Option<&V> {
        for (i, _array_size) in self.arrays.iter().enumerate() {
            let probes = self.probe_sequence(key, i);
            for probe in probes {
                if let Some((stored_key, value)) = &self.buckets[probe] {
                    if stored_key == key {
                        return Some(value); // key found
                    }
                }
            }
        }
        None // key not found
    }
}

pub enum Error {
    KeyAlreadyInserted,
    TableFull,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hash_table = ElasticHashTable::<&str, &str>::new(16);

        assert!(hash_table.insert("key1", "value1").is_ok(), "Failed to insert key1");
        assert!(hash_table.insert("key2", "value2").is_ok(), "Failed to insert key2");
        assert!(hash_table.insert("key3", "value3").is_ok(), "Failed to insert key3");

        assert_eq!("[None, None, None, None, None, Some((\"key1\", \"value1\")), None, None, None, None, Some((\"key3\", \"value3\")), None, Some((\"key2\", \"value2\")), None, None, None]", format!("{:?}", hash_table.buckets));

        assert_eq!(Some(&"value1"), hash_table.get(&"key1"));
        assert_eq!(Some(&"value2"), hash_table.get(&"key2"));
        assert_eq!(Some(&"value3"), hash_table.get(&"key3"));
        assert_eq!(None, hash_table.get(&"key4"));
    }
}
