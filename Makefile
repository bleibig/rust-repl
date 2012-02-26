RUSTC=rustc
#FLAGS=--warn-unused-imports
RM=rm

all: repl

repl: repl.rc repl_main.rs ast_print.rs ast_eval.rs
	$(RUSTC) $(FLAGS) $<

clean:
	$(RM) -rf repl repl.dSYM
