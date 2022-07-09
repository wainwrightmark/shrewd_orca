use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use once_cell::sync::OnceCell;
use serde::*;
use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive( Store, Clone, Default, PartialEq, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct ResultsState{
    pub data: Rc<Vec<Vec<Term>>> ,
    pub warning: Option<String>
}

static TERMDICT : OnceCell<TermDict> = OnceCell::new();

pub fn get_term_dict() -> &'static TermDict {
    TERMDICT.get_or_init(|| TermDict::from_term_data().unwrap())
}
