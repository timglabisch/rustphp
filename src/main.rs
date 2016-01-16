pub mod calculator1; // synthesized by LALRPOP
pub mod tokens;
pub mod ast;

fn main() {

    println!("{:?}", calculator1::parse_Start("if($a + 2) {}"));

    let valid_if = vec![
        "if($a > 5) {}",
        "if($_acd) {}",
        "if(43) {}",
        "if(43 > 45) {}",
        "if(43 > 45 > $b < $c => 3 =< 2) {}",
        "if ( 43  > 45) {}",
        "if ( 43  > 45);"
    ];

    for v in valid_if {
        assert!(calculator1::parse_Start(v).is_ok());
    }
}
