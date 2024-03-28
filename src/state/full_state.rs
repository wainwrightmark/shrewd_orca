use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::prelude::*;
use crate::language::prelude::*;
use beef::Cow;
use log::debug;
use once_cell::sync::OnceCell;
use serde::*;

use yewdux::prelude::*;

#[cfg(target_arch = "wasm32")]
use yewdux::storage;

#[derive(Clone, Serialize, Deserialize)]
pub struct FullState {
    pub text: String,
    #[serde(skip)]
    pub hot: bool,
    #[serde(skip)]
    pub is_complete: bool,
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
        self.text == other.text
            && self.data.len() == other.data.len()
            && self.is_complete == other.is_complete
            && self.hot == other.hot
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
            hot: true,
            is_complete: true,
            question: None,
            data: Default::default(),
            warning: Default::default(),
            iter: Default::default(),
        }
    }
}

impl Store for FullState {
    fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use yewdux::prelude::init_listener;
            init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));
            let result: Result<Option<FullState>, _> = storage::load(storage::Area::Local);

            let mut fs = match result {
                Ok(opt) => match opt {
                    Some(fs) => fs,
                    None => FullState::default(),
                },
                Err(_) => FullState::default(),
            };

            //log::info!("Listener Init: {}", fs.text);
            fs.hot = true;
            fs.update_if_hot();
            fs
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            FullState::default()
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl FullState {
    pub fn info_text(&self) -> Cow<'static, str> {
        if let Some(w) = &self.warning {
            return w.clone().into();
        }

        if self.hot {
            return "...".into();
        }

        if self.is_complete {
            format!("Found all {} solutions", self.data.len()).into()
        } else {
            format!("Found {} solutions", self.data.len()).into()
        }
    }

    pub fn load_more(&mut self, number_to_load: usize) {
        if self.is_complete {
            return;
        }
        if let Some(iter) = self.iter.borrow() {
            let mut i = 0;
            let start_instant = instant::Instant::now();

            let mut iter_borrow = iter.as_ref().borrow_mut();



            while let Some(s) = iter_borrow.next() {
                self.data.push(s);
                i += 1;
                if i >= number_to_load {
                    break;
                }
            }
            if i < number_to_load {
                self.is_complete = true;
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
            Ok(mut question) => {
                let solve_context = get_solve_context();
                if question.is_too_difficult(solve_context) {
                    self.data.clear();
                    self.iter = None;
                    self.warning = Some("Question is too difficult".to_string());
                    self.is_complete = false;
                } else {
                    question.upgrade_literals(solve_context);
                    let qq = Box::leak(Box::new(question));
                    let iter = qq.solve(solve_context);

                    self.data.clear();
                    self.iter = Some(Rc::new(RefCell::new(iter)));
                    self.warning = Default::default();
                    self.is_complete = false;
                }
            }
            Err(warning) => {
                self.data.clear();
                self.iter = None;
                self.warning = Some(warning.to_string());
                self.is_complete = false;
            }
        }
    }

    pub fn update_if_hot(&mut self) {
        if self.hot {
            self.iter = None;
            self.update();
            self.load_more(10);
            self.hot = false;
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
