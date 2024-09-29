use crate::parsing::ast::Exp;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub spec);

pub fn parse(s: String) -> (Exp, Exp, bool) {
    spec::InputParser::new().parse(&s).unwrap()
}

#[test]
pub fn test() {
    println!("{:?}", spec::BExpParser::new().parse("1").unwrap());
    println!("{:?}", spec::BExpParser::new().parse("Ok123").unwrap());
    println!("{:?}", spec::BExpParser::new().parse("(not 1)").unwrap());
    println!(
        "{:?}",
        spec::BExpParser::new().parse("(or (not 1) 1 0 1)").unwrap()
    );
    println!(
        "{:?}",
        spec::BExpParser::new()
            .parse("(and (not 1) 1 0 1)")
            .unwrap()
    );

    println!("{:?}", spec::ExpParser::new().parse("ok").unwrap());
    println!(
        "{:?}",
        spec::ExpParser::new()
            .parse("(seq a (seq x y z) c d)")
            .unwrap()
    );
    println!("{:?}", spec::ExpParser::new().parse("(test abd)").unwrap());
    println!(
        "{:?}",
        spec::ExpParser::new().parse("(while abd\n b)\n").unwrap()
    );
}
