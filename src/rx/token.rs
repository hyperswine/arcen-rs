use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[token("rx")]
    Rx,
    #[token("@")]
    At,
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
