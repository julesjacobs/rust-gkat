use crate::parsing::ast::Exp;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub spec);

pub fn test() {
    println!("{:?}", spec::BExpParser::new().parse("1").unwrap());
    println!("{:?}", spec::BExpParser::new().parse("Ok123").unwrap());
    println!("{:?}", spec::BExpParser::new().parse("(not 1)").unwrap());
    println!(
        "{:?}",
        spec::BExpParser::new().parse("(or (not 1) ok)").unwrap()
    );

    println!("{:?}", spec::ExpParser::new().parse("ok").unwrap());
    println!(
        "{:?}",
        spec::ExpParser::new().parse("(seq abc abd)").unwrap()
    );
    println!("{:?}", spec::ExpParser::new().parse("(test abd)").unwrap());
    println!(
        "{:?}",
        spec::ExpParser::new().parse("(while abd\n b)\n").unwrap()
    );
}

pub fn parse(s: String) -> Exp {
    spec::ExpParser::new().parse(&s).unwrap()
}
