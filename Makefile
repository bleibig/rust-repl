RUSTC=rustc
RM=rm

all: repl

repl: repl.rc repl_main.rs
	$(RUSTC) $<

clean:
	$(RM) -rf repl repl.dSYM
