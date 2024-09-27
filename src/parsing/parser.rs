use crate::parsing::ast::Exp;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub spec);

pub fn parse(s: String) -> (Exp, Exp) {
    spec::InputParser::new().parse(&s).unwrap()
}
