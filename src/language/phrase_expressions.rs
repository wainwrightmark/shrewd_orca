use itertools::Itertools;

use crate::core::prelude::Expression;
use crate::language::prelude::*;

pub fn get_phrase_expressions()-> Vec<Expression>{
    let expression_strings = vec![
"*",
"the #n",
"a #n + @c*",
"an #n + @v*",

"the #j #n",
"a #j + @c* #n",
"an #j + @v* #n",
"#a #v",
    ];

    expression_strings.into_iter()
    .map(|s| question_parse(s).unwrap())      
    .map(|s| match s{
        crate::core::prelude::Question::Expression(e) => e,
        crate::core::prelude::Question::Equation(_) => unreachable!(),
    })
    .collect_vec()
}