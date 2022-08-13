// Maybe build.rs can run this function to expand

use log::debug;
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
            log::info!(" range = {:?}", token.1);
        }
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("token = {:?}", token.0);
            println!(" range = {:?}", token.1);
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
