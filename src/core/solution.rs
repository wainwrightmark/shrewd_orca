use itertools::Itertools;
use smallvec::SmallVec;
use crate::language::prelude::*;
use crate::core::prelude::*;
use std::{collections::BTreeMap, default, str::FromStr};

use crate::language::prelude::*;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Solution{
    pub homographs: SmallVec<[Homograph; 4]>
}

impl Solution{
    pub fn get_text(&self)-> String{
        self.homographs.iter().map(|x|x.text.as_str()).join(" ")
    }
}