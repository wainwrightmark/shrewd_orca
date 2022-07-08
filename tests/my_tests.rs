// use convext::core::prelude::*;

// use ntest::test_case;
// use rand::SeedableRng;
// // use rand::{prelude::StdRng, Rng};

// pub const EXAMPLES: [&str; 8] = [
//     "Circle",
//     "Circle p0.5",
//     "Circle p0.5..0.8",
//     "let myvar 100
// square h ?myvar",
//     "circle circle p 0.5 h 120",
//     "myshape
// rul myshape
// circle
// myshape p 0.75 h 40
// end",
//     "
// blackshape
// rul blackshape
// square h 120
// whiteshape p 0.5 x sub 0.5 y sub 0.5
// whiteshape p 0.5 x 0.5 y 0.5
// end

// rul whiteshape
// square
// blackshape p 0.5 x sub 0.5 y sub 0.5
// blackshape p 0.5 x 0.5 y 0.5
// end",
//     "myshape
// let alpha 0.9
// rul myshape ?h lt 320
// square v 0.5 r?h
// myshape p 0.75 h 10 a?alpha
// end",
// ];

// #[test_case(0)]
// #[test_case(1)]
// #[test_case(2)]
// #[test_case(3)]
// #[test_case(4)]
// #[test_case(5)]
// #[test_case(6)]
// #[test_case(7)]
// fn test_svg(index: usize) {
//     let input = EXAMPLES[index];
//     let grammar = parse(input).unwrap();

//     let mut rng = SeedableRng::seed_from_u64(100);

//     let node = grammar.expand(&ExpandSettings::default(), &mut rng);

//     let svg = node.to_svg(&grammar, &mut rng);

//     assert!(!svg.is_empty());
//     //print!("\r\n{svg}\r\n");
// }
