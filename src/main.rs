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
        assert!(calculator1::parse_simple_variable(v).is_ok());
    }

    println!("{:?}", calculator1::parse_expr("$a += $b"));

}
