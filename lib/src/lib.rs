/*!
This crate provides a library for parsing, compiling, and executing [Lox][] programs.

It is a Rust implementation of the bytecode virtual machine described in the book
[Crafting Interpreters][crafting-interpreters] by [Robert Nystrom][bob].

[crafting-interpreters]: https://craftinginterpreters.com
[lox]: https://craftinginterpreters.com/the-lox-language.html
[bob]: https://stuffwithstuff.com/
*/

pub mod chunk;
pub mod value;
pub mod vm;
