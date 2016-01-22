pub mod calculator1; // synthesized by LALRPOP
pub mod tokens;
pub mod ast;

fn main() {


    // parsing variables
    let valid_simple_variable = vec![
        "$abc",
        "$$abc",
        "${$abc}",
        "${$a += $b}"
    ];

    for v in valid_simple_variable {
        println!("{}", v);
        assert!(calculator1::parse_simple_variable(v).is_ok());
    }

    let expr_without_variable = vec![
        "$a += $b",
        "$a -= $b",
        "$a *= $b",
        "$a /= $b",
        "$a .= $b",
        "$a %= $b",
        "$a &= $b",
        "$a |= $b",
        "$a ^= $b",
        "$a <<= $b",
        "$a >>= $b",
        "$v++",
        "$v--",
        "--$v",
        "++$v"
    ];

    for v in expr_without_variable {
        assert!(calculator1::parse_expr(v).is_ok());
    }

    println!("{:?}", calculator1::parse_expr("$v || $v"));
    println!("{:?}", calculator1::parse_expr("$v || ${$v}"));
    println!("{:?}", calculator1::parse_expr("${$v} || ${$v}"));
    println!("{:?}", calculator1::parse_expr("${$v} || ${$v} || ${$v}"));
    println!("{:?}", calculator1::parse_expr("!${$v} || !!${$v} || ${$v}"));

}
