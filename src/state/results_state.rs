use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
pub struct ImageState {
    pub svg: String,
}

impl Default for ImageState {
    fn default() -> Self {
        let v = Dispatch::<InputState>::new().get();

        let mut s = Self {
            svg: Default::default(),
        };

        s.update_svg(v.as_ref());

        s
    }
}

impl ImageState {
    pub fn update_svg(&mut self, input: &InputState) {
        let mut rng = rand::SeedableRng::seed_from_u64(input.seed);

        let mut override_grammar = input.grammar.clone();
        override_grammar.override_defs(&input.overrides);
        let node = override_grammar.expand(&input.settings, &mut rng);
        let svg = node.to_svg(&override_grammar,&mut rng);
        self.svg = svg;
    }
}
