use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::prelude::*;
use crate::language::prelude::*;
use log::debug;
use once_cell::sync::OnceCell;
use serde::*;
use yewdux::prelude::{init_listener};
use yewdux::storage;
use yewdux::store::Store;

#[derive(Clone, Serialize, Deserialize)]
pub struct FullState {
    pub text: String,
    pub hot: bool,
    #[serde(skip)]
    pub data: Vec<QuestionSolution>,

    #[serde(skip)]
    pub question: Option<Question>,
    pub warning: Option<String>,

    #[serde(skip)]
    pub iter: Option<Rc<RefCell<dyn Iterator<Item = QuestionSolution>>>>,
}

impl PartialEq for FullState {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.data.len() == other.data.len()
    }
}

static SOLVECONTEXT: OnceCell<WordContext> = OnceCell::new();

pub fn get_solve_context() -> &'static WordContext {
    SOLVECONTEXT.get_or_init(WordContext::from_data)
}

impl Default for FullState {
    fn default() -> Self {
        Self {
            text: "hello world =a !phrase".into(),
            hot: false,
            question: None,
            data: Default::default(),
            warning: Default::default(),
            iter: Default::default(),
        }
    }
}

impl Store for FullState {
    fn new() -> Self {
        init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));
        let result: Result<Option<FullState>, _> = storage::load(storage::Area::Local);

        let mut fs = match result {
            Ok(opt) => match opt {
                Some(fs) => fs,
                None => FullState::default(),
            },
            Err(_) => FullState::default(),
        };
        fs.hot = true;
        fs.update_if_hot();
        fs
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl FullState {
    pub fn load_more(&mut self) {
        if let Some(iter) = self.iter.borrow() {
            let mut i = 0;
            let start_instant = instant::Instant::now();

            let mut iter_borrow = iter.as_ref().borrow_mut();
            //let mut data_borrow = self.data.as_ref().borrow_mut();

            while let Some(s) = iter_borrow.next() {
                self.data.push(s);
                i += 1;
                if i >= 10 {
                    break;
                }
            }
            debug!(
                "Found {} solutions ({} total) in {:?}",
                i,
                self.data.len(),
                start_instant.elapsed()
            );
        }
    }

    fn update(&mut self) {
        let r = question_parse(&self.text);
        match r {
            Ok(question) => {
                let qq = Box::leak(Box::new(question));
                let iter = qq.solve(get_solve_context());

                self.data.clear();
                self.iter = Some(Rc::new(RefCell::new(iter)));
                self.warning = Default::default();
            }
            Err(warning) => {
                //debug!("Warning {}", warning);

                self.data.clear();
                self.iter = None;
                self.warning = Some(warning.to_string());
            }
        }
    }

    pub fn update_if_hot(&mut self) {
        if self.hot {
            self.iter = None;
            self.update();
            self.load_more();
        }
    }

    pub fn change_text<S: AsRef<str>>(&mut self, s: S) {
        if self.text.trim() == s.as_ref().trim() {
        } else {
            self.hot = true;
        }
        self.text = s.as_ref().into();
    }
}
