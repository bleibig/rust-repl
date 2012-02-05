import std::io;
import std::io::reader_util;
import rustc::syntax::{codemap, visit};
import rustc::syntax::parse::parser;
import rustc::driver::{driver, session, diagnostic};
import rustc::back::link;

fn main(args: [str]) {
   let argv0 = args[0];
    let demitter = fn@(_cmsp: option<(codemap::codemap, codemap::span)>,
                       _msg: str, _lvl: diagnostic::level) { };
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
          warn_unused_imports: false};
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
            let command = str::slice(input, 1u, str::byte_len(input));
            run_colon_command(command);
        } else {
            let expr_ast = parser::parse_expr_from_source_str("stdin", @input, cfg,
                                                              sess.parse_sess);
            ast_print::visit_expr(expr_ast, 0u, visit::mk_vt(visitor));
            io::println(ast_eval::value_to_str(ast_eval::eval_expr(expr_ast.node)));
        }
    }
}

fn run_colon_command(_command: str) {
    // TODO
}
