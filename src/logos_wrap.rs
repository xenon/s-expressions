use std::iter::Iterator;

// lalrpop expects a lexer that is an iterator and returns an odd type
// It expects a (usize, Token, usize)
// The usizes specify the start and end of a token in the string
// ie: (START, Token::SomeToken, END)

// Logos only gives us a lexer iterator returning Option<Token>
// So this file makes a new iterator that bridges the two interfaces.

pub struct LexerWrapper<'a, T, E>
where
    T: logos::Logos<'a> + PartialEq,
    E: Clone,
{
    lexer: logos::Lexer<'a, T>,
    lex_err: T,
    par_err: E,
}

impl<'a, T, E> LexerWrapper<'a, T, E>
where
    T: logos::Logos<'a> + PartialEq,
    E: Clone,
{
    pub fn new(lexer: logos::Lexer<'a, T>, lexer_error: T, parser_error: E) -> Self {
        LexerWrapper {
            lexer: lexer,
            lex_err: lexer_error,
            par_err: parser_error,
        }
    }
}

impl<'a, T, E> Iterator for LexerWrapper<'a, T, E>
where
    T: logos::Logos<'a> + PartialEq,
    E: Clone,
{
    type Item = Result<(usize, T, usize), E>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next();
        let span = self.lexer.span();
        match token {
            Some(t) if t == self.lex_err => Some(Err(self.par_err.clone())),
            Some(t) => Some(Ok((span.start, t, span.end))),
            None => None,
        }
    }
}
