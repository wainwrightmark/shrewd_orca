use itertools::Itertools;

use crate::language::prelude::*;
use crate::core::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolveSettings{
    max_solutions: usize
}

impl Default for SolveSettings{
    fn default() -> Self {
        Self { max_solutions: 10 }
    }
}

pub struct SolveContext
{
    pub term_dict: TermDict,
    pub anagram_dict: AnagramDict
}

impl SolveContext{
    pub fn from_data()-> SolveContext{
        let term_dict = TermDict::from_term_data().unwrap();
        let anagram_dict = AnagramDict::from(term_dict.terms.clone().into_iter());

        SolveContext { term_dict, anagram_dict }
    }
}

pub trait Solvable{
    fn solve(&self, dict: & SolveContext, settings: &SolveSettings) -> Vec<Vec<Term>>;
}

impl Solvable for Question
{
    fn solve(&self, dict: & SolveContext, settings: &SolveSettings) -> Vec<Vec<Term>>{
        match self {
            Question::Expression(ex) => ex.solve(dict, settings),
            Question::Equation(eq) => eq.solve(dict, settings),
        }
    }
}

impl Solvable for Expression{
    fn solve(&self, dict: & SolveContext, settings: &SolveSettings) -> Vec<Vec<Term>>{


        if self.words.iter().all(|w|w.is_literal()){
            let text = self.words.iter().map(|wq| match wq{
                WordQuery::Literal(s)=> s,
                _=>unreachable!()
            }).join("sep");

            dict.anagram_dict.solve_for_word(text.as_str(), Default::default()).take(settings.max_solutions).collect_vec()
        }
        else{
            let solutions =  self.words.iter().map(|w|w.solve(&dict.term_dict, settings)).multi_cartesian_product() 
            .take(settings.max_solutions).collect_vec();
    
            solutions
        }

        
    }
}

impl Solvable for Equation{
    fn solve(&self, dict: & SolveContext, settings: &SolveSettings) -> Vec<Vec<Term>>{
        todo!()
    }
}

impl WordQuery{

    pub fn allow(&self,term: &Term)-> bool{
        match self{
            WordQuery::Literal(l) => term.text.eq_ignore_ascii_case(l),
            WordQuery::Any => true,
            WordQuery::Range { min, max } => term.text.len() >= *min && term.text.len() <= *max,
            WordQuery::Length(len) => term.text.len() == *len,
            WordQuery::Pattern(p)=> p.allow(term)
        }
    }

    ///Find the solution of there is a single solution
    pub fn find(&self, dict: & TermDict) -> Option<Term>{
        None //TODO implement for Literal
    }

    pub fn solve (&self, dict: & TermDict, settings: &SolveSettings) -> Vec<Term>{
        if let Some(solution) = self.find(dict){
            vec![solution]
        }
        else {
            dict.terms.iter().filter(|t|self.allow(t)).take(settings.max_solutions).cloned().collect_vec()
        }
    }
}



#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use itertools::Itertools;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramDict;
    use super::AnagramKey;
    use crate::core::prelude::*;
    use crate::language::prelude::*;
    use ntest::test_case;

    
    #[test_case("5", 2, "thing; whole", name="length")]
    #[test_case("6 7", 2, "entity benthos; entity someone", name="two lengths")]
    #[test_case("red", 5, "red; red", name="literal")]
    #[test_case("6..7", 3, "entity; object; benthos", name="range")]
    #[test_case("b?d", 6, "bid; bed; bad; bod; bud; bed", name="pattern")]
    #[test_case("b*d", 6, "beachhead; bound; bloodshed; blend; backbend; backhand", name="pattern with any")]

    fn test_solve_with_term_dict(input:String, take: usize, expected: String) {
        let context = SolveContext::from_data();

        let p =  word_lang_parse(input).unwrap();

        let solutions = p.solve(&context, &Default::default());

        let solutions_string = solutions
            .into_iter()
            .sorted_by_key(|x| x.len())
            .take(take)
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .join("; ");

        assert_eq!(solutions_string, expected)
    }
}