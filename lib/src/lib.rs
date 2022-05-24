/*!
This crate provides a library for parsing, compiling, and executing [Lox][] programs.

It is a Rust implementation of the bytecode virtual machine described in the book
[Crafting Interpreters][crafting-interpreters] by [Robert Nystrom][bob].

# Crate features

## Debugging features

 * **`trace`** -
   Enables tracing of the execution of the program.

[crafting-interpreters]: https://craftinginterpreters.com
[lox]: https://craftinginterpreters.com/the-lox-language.html
[bob]: https://stuffwithstuff.com/
*/

pub mod compiler;
pub mod scanner;
pub mod value;
pub mod vm;
