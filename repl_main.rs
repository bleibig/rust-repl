import std::io;
import std::io::reader_util;
import rustc::syntax::{codemap, visit};
import rustc::syntax::parse::parser;
import rustc::driver::{driver, session, diagnostic};
import rustc::back::link;
import ast_eval::to_str;

fn main(args: [str]) {
    monitor {|demitter|
        run_repl(args, demitter);
    };
}

// copypasta from driver::rustc::montor since it cannot be imported
fn monitor(f: fn~(diagnostic::emitter)) {
    enum monitor_msg {
        fatal,
        done,
    };

    let p = comm::port();
    let ch = comm::chan(p);

    alt task::try  {||

        // The 'diagnostics emitter'. Every error, warning, etc. should
        // go through this function.
        let demitter = fn@(cmsp: option<(codemap::codemap, codemap::span)>,
                           msg: str, lvl: diagnostic::level) {
            if lvl == diagnostic::fatal {
                comm::send(ch, fatal);
            }
            diagnostic::emit(cmsp, msg, lvl);
        };

        resource finally(ch: comm::chan<monitor_msg>) {
            comm::send(ch, done);
        }

        let _finally = finally(ch);

        f(demitter)
    } {
        result::ok(_) { /* fallthrough */ }
        result::err(_) {
            // Task failed without emitting a fatal diagnostic
            if comm::recv(p) == done {
                diagnostic::emit(
                    none,
                    diagnostic::ice_msg("unexpected failure"),
                    diagnostic::error);
                let note = "The compiler hit an unexpected failure path. \
                            This is a bug. Try running with \
                            RUST_LOG=rustc=0,::rt::backtrace \
                            to get further details and report the results \
                            to github.com/mozilla/rust/issues";
                diagnostic::emit(none, note, diagnostic::note);
            }
            // Fail so the process returns a failure code
            fail;
        }
    }
}

fn run_repl(args: [str], demitter: diagnostic::emitter) {
   let argv0 = args[0];
    let options: @session::options =
        @{crate_type: session::unknown_crate,
          static: false,
          optimize: 0u,
          debuginfo: false,
          extra_debuginfo: false,
          verify: false,
          lint_opts: [],
          save_temps: false,
          stats: false,
          time_passes: false,
          time_llvm_passes: false,
          output_type: link::output_type_none,
          addl_lib_search_paths: [],
          maybe_sysroot: option::none,
          target_triple: driver::host_triple(),
          cfg: [],
          test: false,
          parse_only: false,
          no_trans: true,
          no_asm_comments: true,
          monomorphize: false,
          warn_unused_imports: false,
          enforce_mut_vars: false};
    let sess = driver::build_session(options, "-", demitter);
    let cfg = driver::build_configuration(sess, argv0, "-");
    let visitor: visit::visitor<uint> = ast_print::mk_visitor();
    let stdin = io::stdin();

    while true {
        io::print("rust> ");
        let raw_input = stdin.read_line();
        if str::is_empty(raw_input) {
            if stdin.eof() {
                io::println("");
                break;
            }
            cont;
        }
        let input = str::trim(raw_input);
        if input[0] == ':' as u8 {
            let command = str::slice(input, 1u, str::len(input));
            run_colon_command(command);
        } else {
            let expr_ast = parser::parse_expr_from_source_str("stdin", @input, cfg,
                                                              sess.parse_sess);
            ast_print::visit_expr(expr_ast, 0u, visit::mk_vt(visitor));
            io::println(ast_eval::eval_expr(expr_ast.node).to_str());
        }
    }
}

fn run_colon_command(_command: str) {
    // TODO
}
