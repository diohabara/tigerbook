# Chapter 2

**Lexical Analysis**

```text
lex-i-cal: of or relating to words or the vocabulary of a language as distinguished from its grammar and construction
Webster's Dictionary
```

This chapter was about lexing, or tokenizing.

Although code samples is written for OCaml/ml-lex, I am writing in Rust. This means that I need to write lexer for my own.

[Rust's lexer itself](https://github.com/rust-lang/rust/tree/790d19cd259e9ec656c6e0bb1446e980115ba46f/compiler/rustc_lexer) seems useful. [That of parser](https://github.com/rust-lang/rust/tree/790d19cd259e9ec656c6e0bb1446e980115ba46f/compiler/rustc_parse) is here. I can take advantage of it in the next chapter.