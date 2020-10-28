# Chapter 1

This chapter was just an introduction.

I wrote [a simple program](https://github.com/diohabara/SLPL), without even a lexer.

It was also simple to write this program. Since Rust can deal with algebraic data types, I just translated OCaml `datatype` into `enum`.

Creating recursive enums require using `Box<T>`. `Box<T>` is a smart pointer to a head allocated `T` values. At compile time, Rust needs to know how much space a type takes up[^1], so you should use `Box<T>` to translate straightforwardly.

[^1]: https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
