use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::{core::prelude::*};


pub struct AnagramIterator<'b>
//TODO const N
{
    dict: &'b AnagramDict,
    stack: SmallVec<[(AnagramKey, Bound<AnagramKey>); 5]>,
    used_words: SmallVec<[AnagramKey; 5]>,
    settings: AnagramSettings,
}

impl<'b> AnagramIterator<'b> {
    pub fn create(dict: &'b AnagramDict, key: AnagramKey, settings: AnagramSettings) -> Self {
        let mut stack = SmallVec::<[(AnagramKey, Bound<AnagramKey>); 5]>::new();
        stack.push((key, Bound::Included(key)));

        Self {
            dict,
            stack,
            settings,
            used_words: Default::default(),
        }
    }
}

impl<'b> Iterator for AnagramIterator<'b> {
    type Item = SmallVec<[AnagramKey; 5]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let top = self.stack.last_mut().unwrap();

            if let Some((remainder, next_key)) = self
                .dict
                .words
                .range((Bound::Unbounded, top.1))
                .rev()
                .filter(|(&next_key, _)| self.settings.allow(&next_key))
                .filter_map(|(&next_key, _)| {
                    (top.0 - next_key).map(|remainder| (remainder, next_key))
                })
                .next()
            {       
                top.1 = Bound:: Excluded(next_key);
                
                if remainder.is_empty() {
                    let mut new_used = self.used_words.clone();
                    new_used.push(next_key);                    
                    return Some(new_used);
                } else if self.settings.allow(&remainder) {
                    if self.settings.max_words == self.used_words.len() + 2 {

                        if remainder <= next_key && self.dict.words.contains_key(&remainder)
                            && self.settings.allow(&remainder)
                        {
                            let mut new_used = self.used_words.clone();
                            new_used.push(next_key);
                            new_used.push(remainder);
                            return Some(new_used);
                        }
                        self.used_words.pop();
                        self.stack.pop();
                    } else {
                        self.used_words.push(next_key);
                        self.stack.push((remainder, Bound::Included(next_key) ))
                    }
                }
            } else {
                self.stack.pop();
                self.used_words.pop();
            }
        }

        None
    }
}