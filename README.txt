Project: rust-repl
Author: Brian Leibig

NOTE: This instance of the rust-repl project is essentially abandoned,
as I have went with a new approach of implementing it that requires
deeper integration with the rust codebase, thus the current version
can be found at https://github.com/bleibig/rust and in the src/repl
directory.

== Description ==

This is an interactive interpreter for the Rust programming
language[1] being developed by Mozilla.  It hopes to eventually be a
viable solution to issue #1120[2].  Right now its implementation is
very minimal and bare-bones, it so far only supports binary
expressions of built-in types, vectors, and records.  As the language
and libraries are rapidly evolving, this is meant to work with the
latest git master revision on github.

== Design ==

The repl works by presenting a prompt, reading an expression from
stdin, parsing it to an ast::expr, printing the ast, and then
evaluating and printing the value it evaluates to.  There are three
source files: 

* repl_main.rs contains the main function and loop, plus the code to
  set up and run the parser on what it reads in.  If the first char is
  ":", it reads it as a command sent directly to the interpreter
  similar to the colon commands that ghci has.

* ast_print.rs prints an ast node and all its children by using the
  AST visitor.  I implemented this mainly for my own benefit to see
  clearly what the evaluator was evaluating.

* ast_eval.rs contains the value type that represents a rust value
  that cannot be evaluated any more, plus functions to evaluate an
  ast::expr and convert it to a string to be printed out.

[1] http://www.rust-lang.org/
[2] https://github.com/mozilla/rust/issues/1120
