









// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;
//     use std::str::FromStr;

//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::AnagramDict;
//     use super::AnagramKey;
//     use crate::core::prelude::*;
//     use crate::language::prelude::*;
//     use ntest::test_case;

//     #[test_case("5", 2, "thing; whole", name = "length")]
//     #[test_case("6 7", 2, "entity benthos; entity someone", name = "two lengths")]
//     #[test_case("red", 5, "red; red", name = "literal")]
//     #[test_case("6..7", 3, "entity; object; benthos", name = "range")]
//     #[test_case("b?d", 6, "bid; bed; bad; bod; bud; bed", name = "pattern")]
//     #[test_case(
//         "b*d",
//         6,
//         "beachhead; bound; bloodshed; blend; backbend; backhand",
//         name = "pattern with any"
//     )]

//     fn test_solve_with_term_dict(input: String, take: usize, expected: String) {
//         let context = WordContext::from_data();

//         let p = word_lang_parse(input).unwrap();

//         let solutions = p.solve(&context, &Default::default());

//         let solutions_string = solutions
//             .into_iter()
//             .sorted_by_key(|x| x.homographs.len())
//             .take(take)
//             .map(|s| s.homographs .into_iter().map(|t| t.text).join(" "))
//             .join("; ");

//         assert_eq!(solutions_string, expected)
//     }
// }
