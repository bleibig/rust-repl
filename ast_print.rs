import rustc::syntax::{ast, visit};
import rustc::syntax::visit::vt;
import rustc::syntax::codemap;

const inc: uint = 2u;

fn spaces(amt: uint) -> str {
    let s = "";
    let i = 0u;
    while i < amt {
        s += " ";
        i += 1u;
    }
    ret s;
}

fn visit_mod(m: ast::_mod, sp: codemap::span, id: ast::node_id, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "mod");
    visit::visit_mod(m, sp, id, e + inc, v);
}

fn visit_view_item(vi: @ast::view_item, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt vi.node {
      ast::view_item_use(_, _, _) { io::println(indent + "view_item_use"); }
      ast::view_item_import(_) { io::println(indent + "view_item_import"); }
      ast::view_item_export(_) { io::println(indent + "view_item_export"); }
    }
    visit::visit_view_item(vi, e, v);
}

fn visit_native_item(ni: @ast::native_item, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "native_item");
    visit::visit_native_item(ni, e + inc, v);
}

fn visit_item(i: @ast::item, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt i.node {
      ast::item_const(_, _) { io::println(indent + "item_const"); }
      ast::item_fn(_, _, _) { io::println(indent + "item_fn"); }
      ast::item_mod(_) { io::println(indent + "item_mod"); }
      ast::item_native_mod(_) { io::println(indent + "item_native_mod"); }
      ast::item_ty(_, _) { io::println(indent + "item_ty"); }
      ast::item_enum(_, _) { io::println(indent + "item_enum"); }
      ast::item_res(_, _, _, _, _) { io::println(indent + "item_res"); }
      ast::item_class(_, _, _) { io::println(indent + "item_class"); }
      ast::item_iface(_, _) { io::println(indent + "item_iface"); }
      ast::item_impl(_, _, _, _) { io::println(indent + "item_impl"); }
    }
    visit::visit_item(i, e + inc, v);
}

fn visit_local(l: @ast::local, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "visit_local");
    visit::visit_local(l, e + inc, v);
}

fn visit_block(b: ast::blk, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "visit_block");
    visit::visit_block(b, e + inc, v);
}

fn visit_stmt(s: @ast::stmt, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt s.node {
      ast::stmt_decl(d, _) { io::println(indent + "stmt_decl"); }
      ast::stmt_expr(ex, _) { io::println(indent + "stmt_expr"); }
      ast::stmt_semi(ex, _) { io::println(indent + "stmt_semi"); }
    }
    visit::visit_stmt(s, e + inc, v);
}

fn visit_arm(a: ast::arm, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "visit_arm");
    visit::visit_arm(a, e + inc, v);
}

fn visit_pat(p: @ast::pat, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "visit_pat");
    visit::visit_pat(p, e + inc, v);
}

fn visit_decl(d: @ast::decl, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt d.node {
      ast::decl_local(_) { io::println(indent + "decl_local"); }
      ast::decl_item(_) { io::println(indent + "decl_item"); }
    }
    visit::visit_decl(d, e + inc, v);
}

fn visit_expr(ex: @ast::expr, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt ex.node {
      ast::expr_vec(es, mutability) { io::println(indent + "expr_vec"); }
      ast::expr_rec(flds, base) { io::println(indent + "expr_rec"); }
      ast::expr_call(callee, args, _) { io::println(indent + "expr_call"); }
      ast::expr_tup(elts) { io::println(indent + "expr_tup"); }
      ast::expr_bind(callee, args) { io::println(indent + "expr_bind"); }
      ast::expr_binary(binop, lhs, rhs) {
        alt binop {
          ast::add { io::println(indent + "add"); }
          ast::subtract { io::println(indent + "sub"); }
          ast::mul { io::println(indent + "mul"); }
          ast::div { io::println(indent + "div"); }
          ast::rem { io::println(indent + "rem"); }
          ast::and { io::println(indent + "and"); }
          ast::or { io::println(indent + "or"); }
          ast::bitxor { io::println(indent + "bitxor"); }
          ast::bitand { io::println(indent + "bitand"); }
          ast::bitor { io::println(indent + "bitor"); }
          ast::lsl { io::println(indent + "lsl"); }
          ast::lsr { io::println(indent + "lsr"); }
          ast::asr { io::println(indent + "asr"); }
          ast::eq { io::println(indent + "eq"); }
          ast::lt { io::println(indent + "lt"); }
          ast::le { io::println(indent + "le"); }
          ast::ne { io::println(indent + "ne"); }
          ast::ge { io::println(indent + "ge"); }
          ast::gt { io::println(indent + "gt"); }
        }
      }
      ast::expr_unary(unop, expr) {
        alt unop {
          ast::box(mutability) { io::println(indent + "box"); }
          ast::uniq(mutability) { io::println(indent + "uniq"); }
          ast::deref { io::println(indent + "deref"); }
          ast::not { io::println(indent + "not"); }
          ast::neg { io::println(indent + "neg"); }
        }
      }
      ast::expr_lit(lit) {
        alt lit.node {
          ast::lit_str(str) { io::println(#fmt["%sstr: %s ", indent, str]); }
          ast::lit_int(int, _) { io::println(#fmt["%sint: %d ", indent, int]); }
          ast::lit_uint(uint, _) { io::println(#fmt["%suint: %u ", indent, uint]); }
          ast::lit_float(float, _) { io::println(#fmt["%sfloat: %s ", indent, float]); }
          ast::lit_nil { io::println(indent + "nil"); }
          ast::lit_bool(bool) {
            io::println(#fmt["%sbool: %s ", indent, bool::to_str(bool)]);
          }
        }
      }
      ast::expr_cast(x, t) { io::println(indent + "expr_cast"); }
      ast::expr_if(x, b, eo) { io::println(indent + "expr_if"); }
      ast::expr_while(expr, blk) { io::println(indent + "expr_while"); }
      ast::expr_for(local, expr, blk) { io::println(indent + "expr_for"); }
      ast::expr_do_while(blk, expr) { io::println(indent + "expr_do_while"); }
      ast::expr_loop(bkl) { io::println(indent + "expr_loop"); }
      ast::expr_alt(expr, arms, _) { io::println(indent + "expr_alt"); }
      ast::expr_fn(proto, fn_decl, blk, capture_clause) {
        io::println(indent + "expr_fn");
      }
      ast::expr_fn_block(fn_decl, blk) { io::println(indent + "expr_fn_block"); }
      ast::expr_block(blk) { io::println(indent + "expr_block"); }
      ast::expr_copy(expr) { io::println(indent + "expr_copy"); }
      ast::expr_move(expr1, expr2) { io::println(indent + "expr_move"); }
      ast::expr_assign(lhs, rhs) { io::println(indent + "expr_assign"); }
      ast::expr_swap(expr1, expr2) { io::println(indent + "expr_swap"); }
      ast::expr_assign_op(binop, lhs, rhs) { io::println(indent + "expr_assign_op"); }
      ast::expr_field(expr, ident, tys) { io::println(indent + "expr_filed"); }
      ast::expr_index(expr1, expr2) { io::println(indent + "expr_index"); }
      ast::expr_path(path) { io::println(indent + "expr_path"); }
      ast::expr_addr_of(_, expr) { io::println(indent + "expr_addr_of"); }
      ast::expr_fail(maybe_expr) { io::println(indent + "expr_fail"); }
      ast::expr_break { io::println(indent + "break"); }
      ast::expr_cont { io::println(indent + "cont"); }
      ast::expr_ret(expr) { io::println(indent + "expr_ret"); }
      ast::expr_be(expr) { io::println(indent + "expr_be"); }
      ast::expr_log(int, expr1, expr2) { io::println(indent + "expr_log"); }
      ast::expr_assert(expr) { io::println(indent + "expr_assert"); }
      ast::expr_check(expr_check_mode, expr) { io::println(indent + "expr_check"); }
      ast::expr_if_check(x, b, eo) { io::println(indent + "expr_if_check"); }
      ast::expr_mac(mac) { io::println(indent + "expr_mac"); }
    }
    visit::visit_expr(ex, e + inc, v);
}

fn visit_ty(t: @ast::ty, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt t.node {
      ast::ty_nil { io::println(indent + "ty_nil"); }
      ast::ty_bot { io::println(indent + "ty_bot"); }
      ast::ty_box(_) { io::println(indent + "ty_box"); }
      ast::ty_uniq(_) { io::println(indent + "ty_uniq"); }
      ast::ty_vec(_) { io::println(indent + "ty_vec"); }
      ast::ty_ptr(_) { io::println(indent + "ty_ptr"); }
      ast::ty_rptr(_, _) { io::println(indent + "ty_rptr"); }
      ast::ty_rec(_) { io::println(indent + "ty_rec"); }
      ast::ty_fn(_, _) { io::println(indent + "ty_fn"); }
      ast::ty_tup(_) { io::println(indent + "ty_tup"); }
      ast::ty_path(_, _) { io::println(indent + "ty_path"); }
      ast::ty_constr(_, _) { io::println(indent + "ty_constr"); }
      ast::ty_mac(_) { io::println(indent + "ty_mac"); }
      ast::ty_infer { io::println(indent + "ty_infer"); }
    }
    visit::visit_ty(t, e + inc, v);
}

fn visit_ty_params(tps: [ast::ty_param], &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "ty_params");
    visit::visit_ty_params(tps, e + inc, v);
}

fn visit_constr(operator: @ast::path, sp: codemap::span, id: ast::node_id, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "constr");
    visit::visit_constr(operator, sp, id, e + inc, v);
}

fn visit_fn(fk: visit::fn_kind, decl: ast::fn_decl, body: ast::blk, sp: codemap::span, id: ast::node_id, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    io::println(indent + "fn");
    visit::visit_fn(fk, decl, body, sp, id, e + inc, v);
}

fn visit_class_item(s: codemap::span, p: ast::privacy, cm: ast::class_member, &&e: uint, v: vt<uint>) {
    let indent = spaces(e);
    alt cm {
      ast::instance_var(_, _, _, _) { io::println(indent + "instance_var"); }
      ast::class_method(_) { io::println(indent + "class_method"); }
    }
    visit::visit_class_item(s, p, cm, e + inc, v);
}

fn mk_visitor() -> visit::visitor<uint> {
    ret @{visit_mod: bind visit_mod(_, _, _, _, _),
          visit_view_item: bind visit_view_item(_, _, _),
          visit_native_item: bind visit_native_item(_, _, _),
          visit_item: bind visit_item(_, _, _),
          visit_local: bind visit_local(_, _, _),
          visit_block: bind visit_block(_, _, _),
          visit_stmt: bind visit_stmt(_, _, _),
          visit_arm: bind visit_arm(_, _, _),
          visit_pat: bind visit_pat(_, _, _),
          visit_decl: bind visit_decl(_, _, _),
          visit_expr: bind visit_expr(_, _, _),
          visit_ty: bind visit_ty(_, _, _),
          visit_ty_params: bind visit_ty_params(_, _, _),
          visit_constr: bind visit_constr(_, _, _, _, _),
          visit_fn: bind visit_fn(_, _, _, _, _, _, _),
          visit_class_item: bind visit_class_item(_,_,_,_,_)};
}
