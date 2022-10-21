use itertools::Itertools;
use shrewd_orca::core::prelude::*;
use shrewd_orca::language::prelude::*;

use ntest::test_case;
use smallvec::SmallVec;



#[test_case("clint eastwood =a !phrase")]
#[test_case("red bat =a **")]
#[test_case("#f eastwood =a !phrase")]
#[test_case("cat =a *")]
#[test_case("clint eastwood =a **")]
#[test_case("clint eastwood =a #n #j")]
#[test_case("clint eastwood =a * *")]
#[test_case("name * =a anagram *")]
#[test_case("5")]
#[test_case("5 + #n")]
#[test_case("3 + #n + #v +#j + #a")]
#[test_case("6 7")]
#[test_case("red")]
#[test_case("c?t fl?p")]
#[test_case("6..7")]
#[test_case("b?d")]
#[test_case("b*d")]
#[test_case("#n")]
#[test_case("#v")]
#[test_case("#a")]
#[test_case("#j")]
#[test_case("c@vt")]
#[test_case("#n + #v + #j + #a + 3")]
#[test_case("(world)")]
#[test_case("(world / earth)")]
#[test_case("w* + (world / earth)")]

fn test(input: String) {
    let context = WordContext::from_data();

    let p = question_parse(input).unwrap();

    let solutions = p.solve(&context).take(10);

    let solutions_string = solutions.into_iter().map(|s| s.get_text()).join("; ");

    insta::assert_snapshot!(solutions_string);
}

#[test_case("#j #n", "dishwasher ingrown", "ingrown dishwasher")]

fn test_order_to_allow(query: String, text: String, expected: String) {
    let dict = WordContext::from_data();

    let q = question_parse(query).unwrap();
    let expression = match q {
        Question::Expression(e) => e,
        Question::Equation(_) => unreachable!(),
    };

    let homographs = SmallVec::from_vec(
        text.split_ascii_whitespace()
            .map(|word| dict.try_get(word).unwrap().clone())
            .collect_vec(),
    );

    let solution = ExpressionSolution { homographs };

    let ordered = expression.order_to_allow(solution);

    assert!(ordered.is_some());

    let actual = ordered.unwrap().get_text();

    assert_eq!(expected, actual);
}

// #[test]
// fn test_phrase_expressions(){
//     let any : WordQuery = WordQueryTerm ::Any.into();
//     let any_exp = FixedLengthExpression{words: vec![any]};


//     let the_noun = FixedLengthExpression{words: vec![WordQueryTerm::Literal(Homograph "The".into()), WordQueryTerm::PartOfSpeech(PartOfSpeech::Noun)]};

//     assert_eq!(PHRASEEXPRESSIONS[0],any_exp );
//     assert_eq!(PHRASEEXPRESSIONS[1],any_exp );
// }