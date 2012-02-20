import rustc::syntax::{ast, visit};
import std::io;

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

fn visit_stmt(s: @ast::stmt, &&e: uint, v: visit::vt<uint>) {
    let indent = spaces(e);
    alt s.node {
      ast::stmt_decl(d, _) { io::println(indent + "stmt_decl"); }
      ast::stmt_expr(ex, _) { io::println(indent + "stmt_expr"); }
      ast::stmt_semi(ex, _) { io::println(indent + "stmt_semi"); }
    }
    visit::visit_stmt(s, e + inc, v);
}

fn visit_expr(ex: @ast::expr, &&e: uint, v: visit::vt<uint>) {
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
fn mk_visitor() -> visit::visitor<uint> {
    ret @{
        visit_expr: bind visit_expr(_, _, _),
        visit_stmt: bind visit_stmt(_, _, _)
        with *visit::default_visitor::<uint>()
     };
}
