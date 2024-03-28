use std::{
    fmt::{Debug, Display, Write},
    marker::PhantomData,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::anyhow;
use prime_bag::PrimeBag128;
use quick_xml::se;

use super::prelude::Character;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnagramKey {
    pub len: u8,
    pub inner: prime_bag::PrimeBag128<Character>,
}

impl Ord for AnagramKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl PartialOrd for AnagramKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl AnagramKey {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // pub const EMPTY: AnagramKey = AnagramKey {
    //     inner: PrimeBag128(1, PhantomData),
    //     len: 0,
    // };

    pub fn empty()-> Self{
        Self{
            len: 0,
            inner: Default::default()
        }
    }

    // pub const PRIMESBYSIZE: [usize; 26] = [
    //     2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    //     97, 101,
    // ];
    // pub const LETTERSBYFREQUENCY: [char; 26] = [
    //     'e', 't', 'a', 'i', 'n', 'o', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'f', 'w', 'y', 'g',
    //     'p', 'b', 'v', 'k', 'q', 'j', 'x', 'z',
    // ];

    // pub const PRIMESBYLETTER: [usize; 26] =
    //     array_const_fn_init::array_const_fn_init![prime_for_letter; 26];
}

impl Add for AnagramKey {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.inner.try_sum(&rhs.inner)?;
        let len = self.len + rhs.len;
        AnagramKey { inner, len }.into()
    }
}

impl Sub for AnagramKey {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.inner.try_difference(&rhs.inner)?;
        let len = self.len - rhs.len;
        AnagramKey { inner, len }.into()
    }
}

impl Debug for AnagramKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = format!("{self}");

        f.debug_struct("AnagramKey")
            .field("txt", &display)
            .field("len", &self.len)
            .field("inner", &self.inner)
            .finish()
    }
}

impl Display for AnagramKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            f.write_char('!')?;
        } else {
            for char in self.inner.into_iter() {
                f.write_char(char.as_char())?;
            }
        }

        return std::fmt::Result::Ok(());
    }
}

impl FromStr for AnagramKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let mut len: u8 = 0;

        let s = s.to_ascii_lowercase();

        let chars = s
            .chars()
            .filter(|c| c.is_ascii_lowercase())
            .flat_map(Character::try_from).inspect(|_|len += 1);

        let inner = PrimeBag128::try_from_iter(chars).ok_or(anyhow!("String is too long"))?;

        Ok(AnagramKey { inner, len })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramKey;

    #[test]
    fn test_anagram_keys() {
        let clint_eastwood = AnagramKey::from_str("clint eastwood").unwrap();
        let old_west_action = AnagramKey::from_str("old west action").unwrap();
        assert_eq!(clint_eastwood, old_west_action);
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
    fn test_sub() {
        let old_west_action = AnagramKey::from_str("old west action").unwrap();

        let eastwood = AnagramKey::from_str("eastwood").unwrap();

        let clint = AnagramKey::from_str("clint").unwrap();

        let subbed = (old_west_action - eastwood).unwrap();

        assert_eq!(clint, subbed);
    }
}
