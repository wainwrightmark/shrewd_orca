use std::{collections::{BTreeMap, HashMap}, str::FromStr,iter::{Once, FlatMap}, ops::Bound, future::Future};
use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;

use crate::core::prelude::*;

pub struct AnagramDict<'a>{
    pub words: BTreeMap<AnagramKey, Vec<Term<'a>>>
}

impl<'a> From<TermDict<'a>> for AnagramDict<'a>{
    fn from(term_dict: TermDict<'a>) -> Self {
        let terms = term_dict.terms;

        Self::from(terms.into_iter())
    }
}

impl<'a, T: Iterator<Item = Term<'a>>> From<T> for AnagramDict<'a>{
    fn from(iter: T) -> Self {

        let groups = iter
        .sorted()
        .dedup()
        .filter_map(|term| AnagramKey::from_str(term.text).ok().map(|key|(key, term)) )
        .into_group_map()
        ;
        let words = BTreeMap::from_iter(groups);
        
        AnagramDict { words }
    }
}



impl<'a> AnagramDict<'a>
{

    fn solve_for_word(&self, word: &str, settings: SolveSettings)-> impl '_ + Iterator<Item = Vec<Term>>{
        let key = AnagramKey::from_str(word).unwrap();
        self.solve(key,settings)
    }

    fn solve(&self, key: AnagramKey, settings: SolveSettings )-> impl '_ + Iterator<Item = Vec<Term>>
     {
        let iterator = AnagramIterator::create(self, key, settings);

        let solutions = iterator.flat_map(|solution| solution.into_iter().map(|k| self.words.get(&k).unwrap().clone()).multi_cartesian_product() );

        solutions
    }

}

#[derive(Clone, Copy, Debug)]
pub struct SolveSettings{
    min_word_length: usize
}

impl Default for SolveSettings{
    fn default() -> Self {
        Self { min_word_length: 3 }
    }
}


pub struct AnagramIterator<'a, 'b> //TODO const N
{
    dict : &'b AnagramDict<'a>,
    stack : SmallVec::<[(AnagramKey,AnagramKey,  SmallVec::<[AnagramKey; 5]> ); 5]>,
    settings: SolveSettings
}

impl<'a, 'b>  AnagramIterator<'a, 'b>{

    pub fn create(dict : &'b AnagramDict<'a>, key : AnagramKey, settings: SolveSettings ) -> Self{
        let mut stack = SmallVec::<[(AnagramKey,AnagramKey,  SmallVec::<[AnagramKey; 5]> ); 5]> ::new();
        stack.push((key, AnagramKey::EMPTY, SmallVec::<[AnagramKey; 5]>::new()));

        Self { dict, stack, settings }
    }
}

impl<'a, 'b> Iterator for AnagramIterator<'a, 'b>{
    type Item =  SmallVec<[AnagramKey; 5]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            if let Some( ( current_key,previous, used )) = self.stack.last_mut(){
    
                if previous >= current_key {//todo check previous squared
                    self.stack.pop();
                    continue;
                }
    
                if let Some((remainder, next_key)) = self.dict.words
                .range((Bound::Excluded(*previous), Bound::Included(*current_key)))
                .filter_map(|(&next_key, terms)| (*current_key - next_key)
                .map(|remainder| (remainder, next_key)) ) .next(){
                    previous.inner  = next_key.inner;                  

                    
                    if remainder.is_empty(){           
                        let mut new_used = used.clone();
                        new_used.push(next_key);             
                        return Some(new_used);
                    }
                    else if next_key > remainder {
                        // if the remainder is in the dictionary, we have already passed it
                    }
                    else{
                        let mut new_used = used.clone();
                        new_used.push(next_key);
                        self.stack.push((remainder, AnagramKey{inner: next_key.inner - 1}, new_used))
                    }
    
                    
                }
                else {
                    self.stack.pop();
                }
            }
        }

        return None;
    }
}

    



#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use itertools::Itertools;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramKey;
    use super::AnagramDict;
    use crate::core::prelude::*;

    #[test]
    fn test_solve() {
        let words= include_str!("common_words.txt").split_ascii_whitespace().map(|text| Term{text, tags: Default::default(), is_single_word: true});

        let dict = AnagramDict::from(words);
        let solutions = dict.solve_for_word("clint eastwood", Default::default());

        let solutions_string = solutions.into_iter()
        .sorted_by_key(|x|x.len())
        
        .map(|s|s.into_iter().map(|t|t.text) .join(" ")).join("; ");

        assert!(solutions_string.contains("west old action;"));
    }
    
    #[test]
    fn test_solve_with_term_dict() {
        let term_dict = TermDict::from_term_data().unwrap();

        let dict = AnagramDict::from(term_dict);

        let solutions = dict.solve_for_word("clint eastwood", Default::default());

        let solutions_string = solutions.into_iter().take(10)
        .sorted_by_key(|x|x.len())
        
        .map(|s|s.into_iter().map(|t|t.text) .join(" ")).join("; ");

        assert_eq!(solutions_string, "Ta To Le din cows; Ta To Le din scow; Ta To Le sic down; Ta To Le now disc; Ta To Le own disc; Ta To Le won disc; Ta To Le cos wind; Ta To Li Ned cows; Ta To Li Ned scow; Ta To Li den cows")
    }



    #[test]
    fn test_solve_basic() {
        let words= "act ire cat".split_ascii_whitespace().map(|text| Term{text, tags: Default::default(), is_single_word: true});

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word("i react", Default::default());

        let solutions_string = solutions.into_iter()
        .sorted_by_key(|x|x.len())
        .map(|s|s.into_iter().map(|t|t.text) .join(" ")).join("; ");

        assert_eq!(solutions_string, "ire act; ire cat");
    }
    
    #[test]
    fn test_duplicate_word() {
        let words= "cha".split_ascii_whitespace().map(|text| Term{text, tags: Default::default(), is_single_word: true});

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word("chacha", Default::default());

        let solutions_string = solutions.into_iter()
        .sorted_by_key(|x|x.len())
        .map(|s|s.into_iter().map(|t|t.text) .join(" ")).join("; ");

        assert_eq!(solutions_string, "cha cha");
    }

    
}