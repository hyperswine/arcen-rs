// Maybe build.rs can run this function to expand
use super::expr::{ElementDef, Expr, Literal, ScopeExpr};
use super::token::Token;
use crate::rust_expr;
use log::debug;
use std::ops::Range;

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

    pub fn accept_index(&mut self, token: Token) -> Result<usize, Token> {
        let t = &self.tokens[self.curr_index];

        if token == t.0 {
            // get the index of t
            return Ok(self.curr_index);
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

    pub fn expect_index(&mut self, token: Token) -> usize {
        match self.accept_index(token) {
            Ok(t) => t,
            Err(e) => {
                panic!("Token \"{token:?}\" was not expected... \"{e:?}\" was the actual token")
            }
        }
    }

    pub fn substr(&self, range_start: usize, range_end: usize) -> String {
        self.input_string[range_start..range_end].to_owned()
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
    // check for "@"
    parser.accept(Token::At)?;

    // expect a scope expression
    parser.expect(Token::CurlyBracketLeft);

    // take what ever is in the scope, a list of child elements

    parser.expect(Token::CurlyBracketRight);

    expr_none!()
}

pub fn element_expr(parser: &mut Parser) -> ExprRes<ElementDef> {
    let ident = parser.accept(Token::Identifier)?;

    // if any params, take em
    if parser.accept_ok(Token::ParenthesisLeft) {
        while let Ok(arg_ident) = parser.accept(Token::Identifier) {
            // let val be the answer of the expression
            // an arg's value should match its type. Which is usually a rust like type, either a single var, tuple or array
            // it can be an rx expression, which is pretty much a literal, identifier or string
            // or it can be a rust expression, which would highlighted by braces {}

            // e.g. Box(height="2rem"), Box(h=2) are valid forms
            // Box(height={var}) means you want to use a rust variable

            // expect
            parser.expect(Token::Equals);

            if let Ok(scope) = scope_expr(parser) {
                // interpret the expression body as a string
            } else {
                // an rx expression or prob just literal/ident
                let val = literal(parser).unwrap();
            }
        }

        parser.expect(Token::ParenthesisRight);
    }

    expr_none!()
}

/// Looks for a generic expression like those found in arg values (positional) and {rust_scope_expr} for injecting rust code into the view. E.g. map(|f| Box{})
pub fn expr(parser: &mut Parser) -> ExprRes<Expr> {
    expr_none!()
}

pub fn scope_expr(parser: &mut Parser) -> ExprRes<ScopeExpr> {
    let start = parser.accept_index(Token::CurlyBracketLeft)?;

    // theres no "commas" or in rx, everything is a self contained statement
    // keep parsing statements, either identifiers, literals, or other rx_elements

    // expect a rust expr, although we could just expect whitespace and not deal with it
    // uhh, idk is it possible to invoke syn here for the next token?
    // we want either an identifier or a path or an expression

    // need to either get next symbol and its span (string) or simply try to "parse" the stuff as a scoped expr and pass it to rust_expr!

    // check for an expression, and also get the span of the left curly to right curly
    // how?. Maybe take note of index of left, then take note of right. Then ask parser for the substr

    // rust_expr!();

    let expr = expr(parser).unwrap();

    let end = parser.expect_index(Token::CurlyBracketRight);

    let string = parser.substr(start, end);

    Ok(ScopeExpr { expr, string })
}

// a literal should be either a double quoted string or int
pub fn literal(parser: &mut Parser) -> ExprRes<Literal> {
    expr_none!()
}

/// Parse tokens (e.g. an rx input string) into a rust string
/// First converts the rx found into AST's. Then converts each AST into their rust forms and replaces their original spans with the rust
pub fn parse_to_rust(parser: &mut Parser) -> String {
    // maybe a parse impl instead? for syn
    // also proc_macro2 somewhere

    // check if its rx'able

    // otherwise, prob just rust. Hand off to rust parser
    String::from("")
}

// We want to recursively look for instances of RX in the input string
// There may be a rust part that contains an rx part, which contains a rust part, which contains an rx part...
