Description
===========

This is an interactive read-eval-print-loop (REPL) for the Rust
programming language[1] being developed by Mozilla. It hopes to
eventually be a viable solution to issue #1120[2]. It works by
presenting a prompt, reading input, then integraing with the compiler
to compile it to a LLVM module which is then executed by the JIT
compiler. If an expression was given, it will evaluate and print its
result.

Some portions of the implementation were based on another repl by
dbp[3], but unlike his version, this aims to be more tightly
integrated with the compiler, and keep everything in memory rather
than use temporary files.

As the language and libraries are rapidly evolving, this is meant to
work with the latest git master revision of rust on github.

Current Status
==============

This should work fine on Linux. On OS X 10.8, the LLVM JIT will fail to execute due to a symbol resolution error, like the following:

    LLVM ERROR: Program used external function '__ZN4repr14__extensions__9meth_512712visit_constr17_2ecc45fa2680b4dc3_05E' which could not be resolved!

Also, the sysroot is manually set to "/usr/local/", if this is not
correct for your machine, change the `maybe_sysroot` value in the
options definition in `run_input()`.

Future Goals
============

* Use readline or something similar for getting input, managing history etc.
* Identify input type based on how it parses instead of str::starts_with
* Implement commands for the REPL similar to how GHCi handles :commands

[1] http://www.rust-lang.org/  
[2] https://github.com/mozilla/rust/issues/1120  
[3] https://github.com/dbp/rustrepl  
