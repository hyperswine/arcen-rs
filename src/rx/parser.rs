// Maybe build.rs can run this function to expand

use logos::Logos;
use std::ops::Range;

/// Parse rx
pub fn parse_rx(rx: &str) {
    // maybe a parse impl instead? for syn
    // also proc_macro2 somewhere
}

// parse tags
// any embedded rust {} blocks gets parsed by the macro engine that parses those specific blocks
// each element becomes a struct with a render view
// a render view is a partial spec that can be executed by the cpu or gpu compute unit to create the command buffer to actually render that somewhere

// Everything is a node
// <Something> gets converted into a node
// <Something attr={}> attr

// logos => match <> angle brackets maybe
// and </>
// then parse the actual identifier

// #[logos(subpattern xdigit = r"[0-9a-fA-F]")]

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("<")]
    AngleBracketLeft,
    #[token(">")]
    AngleBracketRight,
    #[token("{")]
    CurlyBracketLeft,
    #[token("}")]
    CurlyBracketRight,
    #[regex(r"[_a-zA-Z]\w*")]
    Identifier,
    #[regex(r"-?[0-9]+")]
    Int,
    #[regex(r"-?([0-9]+\.[0-9]*|\.[0-9]+)")]
    Float,
    #[regex("\"(?:[^\"]|\\.)*\"")]
    DoubleQuotedString,
    // a literal should be either a double quoted string or int
    #[regex("\x26")]
    EOF,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

impl Token {
    pub fn tokenise(input: &str) -> Vec<(Token, Range<usize>)> {
        let tokens = Token::lexer(input);
        tokens.spanned().collect()
    }
}

pub struct Parser {
    tokens: Vec<(Token, Range<usize>)>,
    curr_index: usize,
    input_string: String,
}

impl Parser {
    pub fn new(
        tokens: Vec<(Token, Range<usize>)>,
        curr_index: usize,
        input_string: String,
    ) -> Self {
        Self {
            tokens,
            curr_index,
            input_string,
        }
    }
}

// wait do I want to parse the beginning of {} as a token tree?
// how do I reuse rust's stuff? maybe
// if you do that you wont get auto formatting
// so you will have to modify rust-analyzer and rustfmt either way
// unless using proc macro to expand the #[component]
// but then cant use () rsx anywhere else like a let

// <Component attr={"Hi"}/>
// <Component attr={var}/>
// {} -> parse as rust scope stmt* expr* or tt*
// take the span from {..}