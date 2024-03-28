use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::anyhow;
use prime_bag::PrimeBag128;

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
        Some(self.cmp(other))
    }
}

impl AnagramKey {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn empty() -> Self {
        Self {
            len: 0,
            inner: Default::default(),
        }
    }
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

        std::fmt::Result::Ok(())
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
            .flat_map(Character::try_from)
            .inspect(|_| len += 1);

        let inner = PrimeBag128::try_from_iter(chars).ok_or(anyhow!("String is too long"))?;

        Ok(AnagramKey { inner, len })
    }
}

#[cfg(test)]
mod tests {
    use super::AnagramKey;
    use std::str::FromStr;

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
