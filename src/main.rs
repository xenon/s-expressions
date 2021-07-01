use lalrpop_util::*;
use logos::{Lexer, Logos};
use std::io::{self, BufRead};

mod ast;
mod logos_wrap;

// this imports the lalrpop parser from src/grammar.lalrpop
lalrpop_mod!(pub grammar);

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex("[1-9][0-9]*|0", nat)]
    Nat(u32),
    #[regex(r"\p{Alphabetic}+", id)]
    Identifier(String),
}

// parse a natural number from a string
fn nat(lex: &mut Lexer<Token>) -> u32 {
    // from_str_radix(char slice, base)
    return match u32::from_str_radix(lex.slice(), 10) {
        Ok(n) => n,
        Err(_) => unreachable!(),
    };
}

// parse an identifier from a string
fn id(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    // nothing here
    LexerError,
}

fn main() {
    let stdin = io::stdin();
    let g = grammar::ExprParser::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let lexer = Token::lexer(&l);
                let parse = g.parse(logos_wrap::LexerWrapper::new(
                    lexer,
                    Token::Error,
                    LexerError::LexerError,
                ));
                println!("Raw AST: {:?}", parse);
                match parse {
                    Ok(expr) => println!("Expression: {}", expr),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            Err(e) => println!("Line Error?: {}", e),
        }
    }
}
