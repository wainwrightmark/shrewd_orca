use itertools::Itertools;
use shrewd_orca::core::prelude::*;
use shrewd_orca::language::prelude::*;

 use ntest::test_case;

    
 
 #[test_case("davis ephanie  =a")]
 #[test_case("clint eastwood =a #n #j")]
 #[test_case("clint eastwood =a * *")]
 #[test_case("name * =a anagram *")] 
 #[test_case("5")]
 #[test_case("5 + #n")]
 #[test_case("3 + #n + #v +#j + #a")]
    #[test_case("6 7")]
    #[test_case("red")]
    #[test_case("c?t fl?p")]
    #[test_case("clint eastwood")]
    #[test_case("6..7")]
    #[test_case("b?d")]
    #[test_case("b*d")]
    #[test_case("#n")]
    #[test_case("#v")]
    #[test_case("#a")]
    #[test_case("#j")]
    #[test_case("cat =a *")]
    #[test_case("c@vt")]
    #[test_case("#n + #v + #j + #a + 3")]
    #[test_case("(world)")]
    #[test_case("(world / earth)")]
    #[test_case("w* + (world / earth)")]
    

    fn test(input: String    ) {
        let context = WordContext::from_data(get_phrase_expressions());

        let p = question_parse(input).unwrap();

        let solutions = p.solve(&context, &Default::default());

        let solutions_string = solutions
            .into_iter()
            .map(|s| s.get_text())
            .join("; ");

        insta::assert_snapshot!(solutions_string);
    }
