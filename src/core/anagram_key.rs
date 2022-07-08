use std::{ops::{Add, Sub}, str::FromStr, num, fmt::{Display, Debug, Write}};
use itertools::{self, Itertools};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnagramKey{
    pub inner: u128
}

impl AnagramKey{
    pub fn is_empty(self: &Self)-> bool{
        self.inner == 1
    }

    pub const EMPTY : AnagramKey = AnagramKey{inner: 1};

    pub const PRIMESBYSIZE : [usize; 26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,101];
    pub const LETTERSBYFREQUENCY : [char; 26] = ['e','t','a','i','n',
    'o','s','h','r','d',
    'l','u','c','m','f',
    'w','y','g','p','b',
    'v','k','q','j','x',
    'z'];  
    

    pub const PRIMESBYLETTER : [usize; 26] = array_const_fn_init::array_const_fn_init![prime_for_letter; 26];
}

const fn prime_for_letter(i: usize) -> usize {

    let a = 'a' as usize;
    let c = i + a;
    let mut index = 0;
    while index < AnagramKey::LETTERSBYFREQUENCY.len() {
        if (AnagramKey::LETTERSBYFREQUENCY[index] as usize) == c   {
            return AnagramKey::PRIMESBYSIZE[index];
        }
        index+=1;
    }
    unreachable!()
}

impl Add for AnagramKey{
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.inner.checked_mul(rhs.inner)?;
        AnagramKey{inner}.into()
    }
}

impl Sub for AnagramKey{
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {

        if rhs.inner == 0 {return None;}

        if self.inner % rhs.inner != 0{return None;}

        let inner = self.inner / rhs.inner;
        AnagramKey{inner}.into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnagramKeyErr{
    WordTooBig
}

impl Display for AnagramKey{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rem = self.inner;

        if rem > 1{

            let mut i = 0;
            for p in AnagramKey::PRIMESBYLETTER{
               
                while rem % (p as u128) == 0{
                    let c = ('a' as u8) + i;
                    f.write_char(c as char)?;
                    rem /= p as u128;

                    if rem == 1{
                        return std::fmt::Result::Ok(());
                    }
                }

                i+=1;
            }
            unreachable!()
        }
        else{
            return f.write_char('!');            
        }

        
    }
}

impl FromStr for AnagramKey{
    type Err = AnagramKeyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner: u128 = 1;        

        for c in s.to_ascii_lowercase().chars().filter(|c|c.is_ascii_lowercase()){
            let i = c as usize - 'a' as usize;
            let prime = AnagramKey::PRIMESBYLETTER[i];
            let r = inner.checked_mul(prime as u128);

            match r {
                Some(p) => inner = p,
                None => return Err(AnagramKeyErr::WordTooBig),
            }        
        }

       Ok(AnagramKey{inner})
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramKey;

    #[test]
    fn test_anagram_keys() {

        let clint_eastwoord = AnagramKey::from_str("clint eastwood").unwrap();
        let old_west_action = AnagramKey::from_str("old west action").unwrap();
        assert_eq!(clint_eastwoord, old_west_action);
    }
    
    #[test]
    fn test_add() {

        let clint = AnagramKey::from_str("clint").unwrap();
        let eastwood = AnagramKey::from_str("eastwood").unwrap();

        let clint_eastwood = (clint + eastwood).unwrap();

        let old_west_action = AnagramKey::from_str("old west action").unwrap();
        assert_eq!(clint_eastwood, old_west_action);
    }

    #[test]
    fn test_sub(){
        let old_west_action = AnagramKey::from_str("old west action").unwrap();

        let eastwood = AnagramKey::from_str("eastwood").unwrap();

        let clint = AnagramKey::from_str("clint").unwrap();

        let subbed = (old_west_action - eastwood).unwrap();

        assert_eq!(clint, subbed);
    }

    
}