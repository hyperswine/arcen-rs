// Maybe build.rs can run this function to expand

use log::debug;
use logos::Logos;
use std::ops::Range;

use super::expr::ElementDef;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[token("rx")]
    Rx,
    #[token("(")]
    ParenthesisLeft,
    #[token(")")]
    ParenthesisRight,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
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
    #[regex("\x00")]
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

    /// Get the next symbol. Prob shouldn't be used
    pub fn next_sym(&mut self) {
        self.curr_index += 1;
    }

    /// Peek at current token without incrementing. Prob shouldn't be used
    pub fn peek(&mut self, token: Token) -> Result<String, Token> {
        let t = &self.tokens[self.curr_index];

        if token == t.0 {
            let res = Ok(self.input_string[t.1.clone()].to_owned());
            debug!("res = {:?}", res);

            return res;
        }

        Err(t.0)
    }

    pub fn accept(&mut self, token: Token) -> Result<String, Token> {
        let t = &self.tokens[self.curr_index];

        if token == t.0 {
            let res = Ok(self.input_string[t.1.clone()].to_owned());
            debug!("res = {:?}", res);
            self.next_sym();

            return res;
        }

        Err(t.0)
    }

    /// Simply returns true or false
    pub fn accept_ok(&mut self, token: Token) -> bool {
        self.accept(token).is_ok()
    }

    pub fn expect(&mut self, token: Token) -> String {
        match self.accept(token) {
            Ok(t) => t,
            Err(e) => {
                panic!("Token \"{token:?}\" was not expected... \"{e:?}\" was the actual token")
            }
        }
    }

    pub fn log_tokens(&self) {
        for token in &self.tokens {
            log::info!("token = {:?}", token.0);
            log::info!("range = {:?}", token.1);
        }
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("token = {:?}", token.0);
            println!("range = {:?}", token.1);
        }
    }
}

// PARSER

pub type ExprRes<T> = Result<T, Token>;

macro_rules! expr_none {
    () => {
        Err(Token::EOF)
    };
}

pub fn rx(parser: &mut Parser) -> ExprRes<ElementDef> {
    // check for "rx"
    parser.accept(Token::Rx)?;

    expr_none!()
}

// a literal should be either a double quoted string or int
pub fn literal(parser: &mut Parser) {}

/// Parse tokens (e.g. an rx input string) into a rust string
pub fn parse_to_rust(parser: &mut Parser) -> String {
    // maybe a parse impl instead? for syn
    // also proc_macro2 somewhere

    // check if its rx'able

    // otherwise, prob just rust. Hand off to rust parser
    String::from("")
}
