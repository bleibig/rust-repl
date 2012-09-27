use core::io::ReaderUtil;
use rustc::back;
use rustc::driver::{driver, session};
use rustc::front;
use rustc::lib::llvm::llvm;
use rustc::metadata::creader;
use rustc::middle::{freevars, kind, lint, trans, ty, typeck};
use rustc::middle;
use syntax::{ast, codemap, diagnostic, parse, visit};

// cache of definitions etc. read from previous inputs
type ReplSession = {
    view_items: ~[~str],
    definitions: ~[~str],
    stmt: ~str
};

fn main(++args: ~[~str]) {
    let stdin = io::stdin();
    let mut rsess: ~ReplSession = ~{
        view_items: ~[],
        definitions: ~[],
        stmt: ~"",
    };
    loop {
        io::print("rust> ");
        let raw_input = stdin.read_line();
        if str::is_empty(raw_input) {
            if stdin.eof() {
                io::println("");
                break;
            }
            loop;
        }
        let input = str::trim(raw_input);
        if input[0] == ':' as u8 {
            let command = str::slice(input, 1, str::len(input));
            run_colon_command(command);
        } else {
            let demitter = diagnostic::emit;
            rsess = match do task::try |copy rsess| {
                run_input(input, rsess, args[0], demitter)
            } {
                result::Ok(s) => copy s,
                result::Err(_) => rsess,
            };
        }
    }
}
        
fn run_input(input: ~str, rsess: &ReplSession, argv0: ~str,
             demitter: diagnostic::emitter) -> ~ReplSession {
    let newrsess = if str::starts_with(input, ~"extern mod ")
        || str::starts_with(input, ~"use ") {
        ~{ view_items: vec::append_one(rsess.view_items, input),
          stmt: ~"", .. *rsess }
    } else if str::starts_with(input, ~"fn ")
        || str::starts_with(input, ~"let ") {
        ~{ definitions: vec::append_one(rsess.definitions, input),
          stmt: ~"", .. *rsess }
    } else {
        ~{ stmt: input, .. *rsess }
    };

    let options: @session::options = @{
        crate_type: session::unknown_crate,
        binary: ~"repl",
        maybe_sysroot: option::Some(path::Path(~"/usr/local/")),
        .. *session::basic_options()
    };
    let sess = driver::build_session(options, demitter);
    let cfg = driver::build_configuration(
        sess, argv0, driver::str_input(input));
    let wrapped = driver::str_input(wrap(newrsess));
    debug!("parsing");
    let mut crate = driver::parse_input(sess, cfg, wrapped);
    debug!("configuration");
    crate = front::config::strip_unconfigured_items(crate);
    debug!("maybe building test harness");
    crate = front::test::modify_for_testing(sess, crate);
    debug!("expansion");
    crate = syntax::ext::expand::expand_crate(sess.parse_sess, sess.opts.cfg, crate);
    debug!("intrinsic injection");
    crate = front::intrinsic_inject::inject_intrinsic(sess, crate);
    debug!("core injection");
    crate = front::core_inject::maybe_inject_libcore_ref(sess, crate);
    debug!("building lint settings table");
    lint::build_settings_crate(sess, crate);
    debug!("ast indexing");
    let ast_map = syntax::ast_map::map_crate(sess.diagnostic(), *crate);
    debug!("external crate/lib resolution");
    creader::read_crates(sess.diagnostic(), *crate, sess.cstore, sess.filesearch,
                         session::sess_os_to_meta_os(sess.targ_cfg.os),
                         sess.opts.static, sess.parse_sess.interner);
    debug!("language item collection");
    let lang_items = middle::lang_items::collect_language_items(crate, sess);
    debug!("resolution");
    let { def_map: def_map,
         exp_map2: exp_map2,
         trait_map: trait_map } =
        middle::resolve::resolve_crate(sess, lang_items, crate);
    debug!("freevar finding");
    let freevars = freevars::annotate_freevars(def_map, crate);
    debug!("region_resolution");
    let region_map = middle::region::resolve_crate(sess, def_map, crate);
    debug!("region paramaterization inference");
    let rp_set = middle::region::determine_rp_in_crate(sess, ast_map, def_map, crate);
    debug!("typechecking");
    let ty_cx = ty::mk_ctxt
        (sess, def_map, ast_map, freevars, region_map, rp_set, move lang_items, crate);
    let (method_map, vtable_map) = typeck::check_crate(ty_cx, trait_map, crate);
    debug!("const marking");
    middle::const_eval::process_crate(crate, def_map, ty_cx);
    debug!("const checking");
    middle::check_const::check_crate(sess, crate, ast_map, def_map, method_map, ty_cx);
    debug!("privacy checking");
    middle::privacy::check_crate(ty_cx, &method_map, crate);
    debug!("loop checking");
    middle::check_loop::check_crate(ty_cx, crate);
    debug!("alt checking");
    middle::check_alt::check_crate(ty_cx, crate);
    debug!("liveness checking");
    let last_use_map = middle::liveness::check_crate(ty_cx, method_map, crate);
    debug!("borrow checking");
    let (root_map, mutbl_map) = middle::borrowck::check_crate(
        ty_cx, method_map, last_use_map, crate);
    debug!("kind checking");
    kind::check_crate(ty_cx, method_map, last_use_map, crate);
    debug!("lint checking");
    lint::check_crate(ty_cx, crate);
    let maps = {mutbl_map: mutbl_map,
                root_map: root_map,
                    last_use_map: last_use_map,
                    method_map: method_map,
                    vtable_map: vtable_map};
    if newrsess.stmt != ~"" {
        debug!("translation");
        let (llmod, _) = trans::base::trans_crate(
            sess, crate, ty_cx, ~path::from_str("repl_dummy.rc"),
            exp_map2, maps);
        let pm = llvm::LLVMCreatePassManager();
        debug!("executing jit");
        back::link::jit::exec(sess, pm, llmod, 0, false);
        llvm::LLVMDisposePassManager(pm);
    }
    newrsess
}

fn wrap(rsess: &ReplSession) -> ~str {
    let mut s = ~"extern mod std;\n";
    for vec::each(rsess.view_items) |i| {
        s += i + ~";\n";
    }
    s += ~"fn main() {\n";
    for vec::each(rsess.definitions) |d| {
        s += d + ~";\n";
    }
    if rsess.stmt != ~"" {
        s += ~"io::println(fmt!(\"%?\"," + rsess.stmt + ~"));\n";
    }
    s += ~"\n}\n";
    s
}

fn run_colon_command(_command: ~str) {
    // TODO
}
