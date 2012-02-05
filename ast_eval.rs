import rustc::syntax::ast;

enum value {
    strval(str),
    intval(int),
    uintval(uint),
    floatval(float),
    nilval,
    boolval(bool),
    vecval([@value]),
    recval([@field], option<@value>),
    // more to come like functions etc.
}

type field = {ident: str, val: value}; // for recval

fn value_to_str(v: value) -> str {
    alt v {
      strval(s) { s }
      intval(i) { int::str(i) }
      uintval(i) { uint::str(i) }
      floatval(f) { float::to_str(f, 64u) }
      nilval { "()" }
      boolval(b) {
        alt b {
          true { "true" }
          false { "false" }
        }
      }
      vecval(vs) {
        if vec::is_empty(vs) {
            ret "[]";
        } else {
            let s = "[";
            for v in vec::init(vs) {
                s += value_to_str(*v) + ", ";
            }
            s += value_to_str(*option::get(vec::last(vs))) + "]";
            ret s;
        }
      }
      recval(fields, base) {
        let s = "{ ";
        for f in vec::init(fields) {
            s += f.ident + ": " + value_to_str(f.val) + ", ";
        }
        let lastfield = *option::get(vec::last(fields));
        s += lastfield.ident + ": " + value_to_str(lastfield.val);
        alt base {
          option::none { s += " }"; }
          option::some(rec) { s += " with " + value_to_str(*rec) + " }" }
        }
        ret s;
      }
    }
}

fn add_vals(lhs: value, rhs: value) -> value { 
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { strval(s1 + s2) }
      (intval(i1), intval(i2)) { intval(i1 + i2) }
      (uintval(u1), uintval(u2)) { uintval(u1 + u2) }
      (floatval(f1), floatval(f2)) { floatval(f1 + f2) }
      (vecval(v1), vecval(v2)) { vecval(v1 + v2) }
      (_, _) { fail }
    }
}

fn sub_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (intval(i1), intval(i2)) { intval(i1 - i2) }
      (uintval(u1), uintval(u2)) { uintval(u1 - u2) }
      (floatval(f1), floatval(f2)) { floatval(f1 - f2) }
      (_, _) { fail }
    }
}

fn mul_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (intval(i1), intval(i2)) { intval(i1 * i2) }
      (uintval(u1), uintval(u2)) { uintval(u1 * u2) }
      (floatval(f1), floatval(f2)) { floatval(f1 * f2) }
      (_, _) { fail }
    }
}

fn div_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (intval(i1), intval(i2)) { intval(i1 / i2) }
      (uintval(u1), uintval(u2)) { uintval(u1 / u2) }
      (floatval(f1), floatval(f2)) { floatval(f1 / f2) }
      (_, _) { fail }
    }
}

fn rem_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (intval(i1), intval(i2)) { intval(i1 % i2) }
      (uintval(u1), uintval(u2)) { uintval(u1 % u2) }
      (floatval(f1), floatval(f2)) { floatval(f1 % f2) }
      (_, _) { fail }
    }
}

fn and_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (boolval(b1), boolval(b2)) { boolval(b1 && b2) }
      (_, _) { fail }
    }
}

fn or_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (boolval(b1), boolval(b2)) { boolval(b1 || b2) }
      (_, _) { fail }
    }
}
          
fn bitxor_vals(lhs: value, rhs: value) -> value { 
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 ^ i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 ^ u2) }
     (boolval(b1), boolval(b2)) { boolval(b1 ^ b2) }
     (_, _) { fail }
   }
}

fn bitand_vals(lhs: value, rhs: value) -> value { 
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 & i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 & u2) }
     (boolval(b1), boolval(b2)) { boolval(b1 & b2) }
     (_, _) { fail }
   }
}

fn bitor_vals(lhs: value, rhs: value) -> value { 
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 | i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 | u2) }
     (boolval(b1), boolval(b2)) { boolval(b1 | b2) }
     (_, _) { fail }
   }
}

fn lsl_vals(lhs: value, rhs: value) -> value {
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 << i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 << u2) }
     (_, _) { fail }
   }
}

fn lsr_vals(lhs: value, rhs: value) -> value {
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 >> i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 >> u2) }
     (_, _) { fail }
   }
}

fn asr_vals(lhs: value, rhs: value) -> value {
   alt (lhs, rhs) {
     (intval(i1), intval(i2)) { intval(i1 >>> i2) }
     (uintval(u1), uintval(u2)) { uintval(u1 >>> u2) }
     (_, _) { fail }
   }
}

fn eq_vals(lhs: value, rhs: value) -> value { 
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 == s2) }
      (intval(i1), intval(i2)) { boolval(i1 == i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 == u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 == f2) }
      (nilval, nilval) { boolval(() == ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 == b2) }
      (_, _) { fail }
    }
}

fn lt_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 < s2) }
      (intval(i1), intval(i2)) { boolval(i1 < i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 < u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 < f2) }
      (nilval, nilval) { boolval(() < ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 < b2) }
      (_, _) { fail }
    }
}

fn le_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 <= s2) }
      (intval(i1), intval(i2)) { boolval(i1 <= i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 <= u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 <= f2) }
      (nilval, nilval) { boolval(() <= ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 <= b2) }
      (_, _) { fail }
    }
}

fn ne_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 != s2) }
      (intval(i1), intval(i2)) { boolval(i1 != i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 != u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 != f2) }
      (nilval, nilval) { boolval(() != ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 != b2) }
      (_, _) { fail }
    }
}

fn ge_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 >= s2) }
      (intval(i1), intval(i2)) { boolval(i1 >= i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 >= u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 >= f2) }
      (nilval, nilval) { boolval(() >= ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 >= b2) }
      (_, _) { fail }
    }
}

fn gt_vals(lhs: value, rhs: value) -> value {
    alt (lhs, rhs) {
      (strval(s1), strval(s2)) { boolval(s1 > s2) }
      (intval(i1), intval(i2)) { boolval(i1 > i2) }
      (uintval(u1), uintval(u2)) { boolval(u1 > u2) }
      (floatval(f1), floatval(f2)) { boolval(f1 > f2) }
      (nilval, nilval) { boolval(() > ()) }
      (boolval(b1), boolval(b2)) { boolval(b1 > b2) }
      (_, _) { fail }
    }
}

fn eval_expr(e: ast::expr_) -> value {
    alt e {
      ast::expr_vec(exprs, _) {
        vecval(vec::map(exprs, {|ex| @eval_expr(ex.node)}))
      }
      ast::expr_rec(fields, base) {
        recval(vec::map(fields, {|f| @{ident: f.node.ident, val: eval_expr(f.node.expr.node)} }),
               alt base {
                   option::none { option::none }
                   option::some(e) { option::some(@eval_expr(e.node)) }
               })
      }
      ast::expr_lit(lit) {
        alt lit.node {
          ast::lit_str(s) { strval(s) }
          ast::lit_int(i, _) { intval(i) }
          ast::lit_uint(i, _) { uintval(i) }
          ast::lit_float(f, _) { floatval(float::from_str(f)) }
          ast::lit_nil { nilval }
          ast::lit_bool(b) { boolval(b) }
        }
      }
      ast::expr_binary(op, lhs, rhs) {
        alt op {
          ast::add { add_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }          
          ast::subtract { sub_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::mul { mul_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::div { div_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::rem { rem_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::and { and_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::or { or_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::bitxor { bitxor_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::bitand { bitand_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::bitor { bitor_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::lsl { lsl_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::lsr { lsr_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::asr { asr_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::eq { eq_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::lt { lt_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::le { le_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::ne { ne_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::ge { ge_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
          ast::gt { gt_vals(eval_expr(lhs.node), eval_expr(rhs.node)) }
        }
      }
      ast::expr_unary(op, ex) {
        alt op {
          ast::box(_) { fail "boxed values nyi" }
          ast::uniq(_) { fail "unique pointers nyi" }
          ast::deref { fail "deref nyi" }
          ast::not {
            alt eval_expr(ex.node) {
              boolval(b) { boolval(!b) }
              _ { fail }
            }
          }
          ast::neg {
            alt eval_expr(ex.node) {
              intval(i) { intval(-i) }
              floatval(f) { floatval(-f) }
              _ { fail }
            }
          }
        }
      }
      _ { fail "expr nyi" }
    }
}