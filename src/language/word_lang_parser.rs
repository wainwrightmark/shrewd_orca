use std::str::FromStr;

use crate::core::prelude::*;

use itertools::Itertools;
use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "language/wordlang.pest"]
pub struct WordLangParser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type Result<T> = std::result::Result<T, Error<Rule>>;

#[pest_consume::parser]
impl WordLangParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn many_term(input: Node) -> Result<ManyExpressionType> {
        Ok(match_nodes!(input.into_children();
            [many_any(x)] => x,
            [many_tag(x)] => x,
        ))
    }

    fn many_any(input: Node) -> Result<ManyExpressionType> {
        Ok(ManyExpressionType::Any)
    }

    fn many_tag(input: Node) -> Result<ManyExpressionType> {
        if input.as_str().to_ascii_lowercase() == "phrase" {
            Ok(ManyExpressionType::Phrase)
        } else {
            Err(input.error("Not a many expression"))
        }
    }

    fn many_expression(input: Node) -> Result<ManyExpression> {
        Ok(match_nodes!(input.into_children();
            [many_term(t), query_term(terms)..] => ManyExpression { t, terms: terms.collect(), min_words: 1, max_words: None },
        ))
    }
    fn range(input: Node) -> Result<WordQueryTerm> {
        Ok(match_nodes!(input.into_children();
            [length(min), length(max)] => WordQueryTerm::Range{min, max},
        ))
    }
    fn length(input: Node) -> Result<usize> {
        let lit = input.as_str();
        let len = usize::from_str(lit).map_err(|e| input.error(e));

        len
    }
    fn tag(input: Node) -> Result<WordQueryTerm> {
        let lit = input.as_str();

        if let Ok(pos) = PartOfSpeech::from_str(lit) {
            return Ok(WordQueryTerm::PartOfSpeech(pos));
        }

        if let Ok(wordtag) = WordTag::from_str(lit) {
            return Ok(WordQueryTerm::Tag(wordtag));
        }
        Err(input.error("Not a valid tag"))
    }
    fn bracketed_conjunction(input: Node) -> Result<WordQueryTerm> {
        Ok(match_nodes!(input.into_children();
            [word_query_conjunction(wq)] => WordQueryTerm::Nested(wq.into()),
        ))
    }
    fn literal(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }

    fn any(input: Node) -> Result<WordQueryTerm> {
        Ok(WordQueryTerm::Any)
    }

    fn pattern(input: Node) -> Result<Pattern> {
        let components: Vec<PatternComponent> = input
            .into_pair()
            .into_inner()
            .map(PatternComponent::try_parse)
            .try_collect()?;

        Ok(components.into())
    }

    fn query_term(input: Node) -> Result<WordQueryTerm> {
        Ok(match_nodes!(input.into_children();
            [pattern(x)] =>WordQueryTerm::Pattern(x),
            [literal(text)] => WordQueryTerm::Literal(Homograph {
                text,
                is_single_word: true,
                meanings: Default::default(),
            }) ,
            [any(x)] =>x ,
            [range(x)] =>x ,
            [length(x)] =>WordQueryTerm::Length(x),
            [tag(x)] =>x ,
            [bracketed_conjunction(x)] =>x ,
        ))
    }

    fn word_query_disjunction(input: Node) -> Result<WordQueryDisjunction> {
        Ok(match_nodes!(input.into_children();
            [query_term(terms)..] => WordQueryDisjunction{terms: terms.collect()} ,
        ))
    }

    fn word_query_conjunction(input: Node) -> Result<WordQuery> {
        Ok(match_nodes!(input.into_children();
            [word_query_disjunction(terms)..] => WordQuery{terms: terms.collect()} ,
        ))
    }

    fn fixed_length_expression(input: Node) -> Result<FixedLengthExpression> {
        Ok(match_nodes!(input.into_children();
            [word_query_conjunction(words)..] => FixedLengthExpression{words:words.collect() } ,
        ))
    }

    fn expression(input: Node) -> Result<Expression> {
        Ok(match_nodes!(input.into_children();
            [many_expression(me)] => Expression::Many(me)  ,
            [fixed_length_expression(fle)] => Expression::FixedLength(fle) ,
        ))
    }

    fn equality_operator(input: Node) -> Result<EqualityOperator> {
        match input.as_str().to_ascii_lowercase().as_str() {
            "=a" => Ok(EqualityOperator::Anagram),
            "=s" => Ok(EqualityOperator::Spoonerism),
            _ => Err(input.error("Could not parse as equality operator")),
        }
    }

    fn equation(input: Node) -> Result<Equation> {
        Ok(match_nodes!(input.into_children();
            [expression(left), equality_operator(operator), expression(right)] => Equation{left, operator, right},
        ))
    }

    fn question(input: Node) -> Result<Question> {
        Ok(match_nodes!(input.into_children();
            [equation(eq)] => Question::Equation(eq),
            [expression(e)] => Question::Expression(e),
        ))
    }

    fn file(input: Node) -> Result<Question> {
        Ok(match_nodes!(input.into_children();
            [question(q), _] => q,
        ))
    }
}

pub fn question_parse(input_str: &str) -> Result<Question> {
    // Parse the input into `Nodes`
    let inputs = WordLangParser::parse(Rule::file, input_str)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    // Consume the `Node` recursively into the final value
    WordLangParser::file(input)
}

impl PatternComponent {
    fn try_parse(pair: pest::iterators::Pair<Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::question_marks => Ok(PatternComponent::AnyChar(pair.as_str().len())),
            Rule::any => Ok(PatternComponent::Any),
            Rule::literal => Ok(PatternComponent::Literal(pair.as_str().to_string())),
            Rule::character_class => Ok(PatternComponent::CharacterClass(
                CharacterClass::from_str(pair.as_str()).map_err(|x| {
                    Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: x.to_string(),
                        },
                        pair.as_span(),
                    )
                })?,
            )),
            _ => unreachable!(),
        }
    }
}
