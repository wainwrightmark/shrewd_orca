use smallvec::SmallVec;
use std::ops::Bound;

use crate::core::prelude::*;

pub struct AnagramIterator<'b, const N: usize> {
    dict: &'b AnagramDict,
    stack: SmallVec<[(AnagramKey, Bound<AnagramKey>); N]>,
    used_words: SmallVec<[AnagramKey; N]>,
    settings: AnagramSettings,
}

impl<'b, const N: usize> AnagramIterator<'b, N> {
    pub fn create(dict: &'b AnagramDict, key: AnagramKey, settings: AnagramSettings) -> Self {
        let mut stack = SmallVec::<[(AnagramKey, Bound<AnagramKey>); N]>::new();
        stack.push((key, Bound::Included(key)));

        Self {
            dict,
            stack,
            settings,
            used_words: Default::default(),
        }
    }
}

impl<'b, const N: usize> Iterator for AnagramIterator<'b, N> {
    type Item = SmallVec<[AnagramKey; N]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let top = self.stack.last_mut().unwrap();

            if let Some((remainder, next_key)) = self
                .dict
                .words
                .range((Bound::Unbounded, top.1))
                .rev()
                .filter(|(&next_key, _)| {
                    self.settings.allow_key(&next_key)
                    //&& self.settings.allow_word(possible_homographs)
                })
                .filter_map(|(&next_key, _)| {
                    (top.0 - next_key).map(|remainder| (remainder, next_key))
                })
                .next()
            {
                top.1 = Bound::Excluded(next_key);

                if remainder.is_empty() {
                    let mut new_used = self.used_words.clone();
                    new_used.push(next_key);
                    return Some(new_used);
                } else if self.settings.allow_key(&remainder) {
                    if self.settings.max_words  == Some(self.used_words.len() + 2) {
                        if remainder <= next_key && self.dict.words.contains_key(&remainder) {
                            //if(self.settings.allow_word(l))
                            {
                                let mut new_used = self.used_words.clone();
                                new_used.push(next_key);
                                new_used.push(remainder);
                                return Some(new_used);
                            }
                        }
                    } else if self.settings.max_words.is_none() || self.settings.max_words > Some(self.used_words.len() + 2) {
                        self.used_words.push(next_key);
                        self.stack.push((remainder, Bound::Included(next_key)))
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
