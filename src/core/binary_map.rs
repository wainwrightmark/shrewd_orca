use itertools::*;
use smallvec::SmallVec;
use std::{
    collections::BTreeMap,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use super::{anagram_key::AnagramKey, prelude::Homograph};

pub struct BinaryMap<Key, Value, const SIZE: usize> {
    keys: Vec<Key>,
    values: Vec<smallvec::SmallVec<[Value; SIZE]>>,
}

impl<Key: Ord, Value, const SIZE: usize> BinaryMap<Key, Value, SIZE> {
    pub fn get(&self, key: &Key) -> Option<&smallvec::SmallVec<[Value; SIZE]>> {
        match self.keys.binary_search(key) {
            Ok(index) => self.values.get(index),
            Err(_) => None,
        }
    }

    pub fn range(
        &self,
        range: impl RangeBounds<Key>,
    ) -> impl DoubleEndedIterator<Item = (&Key, &smallvec::SmallVec<[Value; SIZE]>)>
    {
        let start_bound = match range.start_bound() {
            Bound::Included(inc) => match self.keys.binary_search(inc) {
                Ok(b) => Bound::Included(b),
                Err(b) => Bound::Included(b),
            },
            Bound::Excluded(exc) => match self.keys.binary_search(exc) {
                Ok(b) => Bound::Excluded(b),
                Err(b) => Bound::Included(b),
            },
            Bound::Unbounded => Bound::Unbounded,
        };

        let end_bound = match range.end_bound() {
            Bound::Included(b) => match self.keys.binary_search(b) {
                Ok(b) => Bound::Included(b),
                Err(b) => Bound::Excluded(b),
            },
            Bound::Excluded(b) => match self.keys.binary_search(b) {
                Ok(b) => Bound::Excluded(b),
                Err(b) => Bound::Excluded(b),
            },
            Bound::Unbounded => Bound::Unbounded,
        };

        let bounds = (start_bound, end_bound);
        let keys = &self.keys[bounds];
        let values = &self.values[bounds];

        keys.iter().zip(values.iter())
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.keys.binary_search(key).is_ok()
    }
}

impl<T: Iterator<Item = Homograph>, const SIZE: usize> From<T>
    for BinaryMap<AnagramKey, Homograph, SIZE>
{
    fn from(iter: T) -> Self {
        let groups = iter
            .sorted()
            .dedup()
            .filter_map(|term| AnagramKey::from_str(&term.text).ok().map(|key| (key, term)))
            .into_group_map();
        let words =
            BTreeMap::from_iter(groups.into_iter().map(|(k, g)| (k, SmallVec::from_vec(g))));

        Self {
            keys: words.keys().cloned().collect_vec(),
            values: words.into_values().collect_vec(),
        }
    }
}

// impl<Key, Value> BinaryMap<Key, Value> {
//     pub fn new(items: impl Iterator<Item = Value>, get_key : Fn(Value)-> Key)-> Self{
//         let mut values : Vec<Value> = items.collect();
//         values.sort_by_cached_key(get_key);
//         let keys : Vec<Key> = values.iter().map(get_key).collect();

//         Self { keys, values }
//     }
// }
