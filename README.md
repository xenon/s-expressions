# S-expressions

Making a program that can parse some basic expressions is an
easy start to learning the tools required to make a compiler.

S-expressions are simple grammar to parse and serve as a good
example for understanding how to rust lexing/parsing libraries.

With this knowledge, one could go further and parse a more
complicated language or perhaps write the interpreter code and
have a simple lisp-like language using this parser.

This project also serves as an introduction to using the lexer
[Logos](https://crates.io/crates/logos/) and the parser generator
[lalrpop](https://crates.io/crates/lalrpop).

## What does it do?
- Read lines of input and attempt to parse them as s-expressions
- If parsing succeeds, print the s-expression structure
- If parsing fails, print and error message

## S-expression grammar 
I decided to use this basic variant of s-expressions.
```
<s-expr>  ::= <atom> | '( <s-exprs> ')'
<s-exprs> ::= <s-expr> <s-exprs> | <s-expr>
<atom>    ::= <identifier> | <nat>
```

### Terminals
- `identifier` is any string
- `nat` is any integer ≥ 0. (must fit in a `u32`...)

### An extension to the s-expression
This project could easily be extended to handle boolean values
in the grammar and would server as a good exercise!

## Crates used:
- Lexer - [Logos](https://crates.io/crates/logos/)
- Parser - [lalrpop](https://crates.io/crates/lalrpop)

## Difficulties
- Getting the two crates to work together
- Being careful with where enums require boxes in the grammar
- Understanding the syntax for a list in lalrpop grammar

# License
- Unlicense
- See LICENSE for more information
