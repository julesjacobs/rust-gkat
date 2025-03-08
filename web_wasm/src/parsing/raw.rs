#[derive(Debug, Clone)]
pub enum BExp {
    Zero,
    One,
    PBool(String),
    Or(Box<BExp>, Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Not(Box<BExp>),
}

#[derive(Debug, Clone)]
pub enum Exp {
    Act(String),
    Seq(Box<Exp>, Box<Exp>),
    Ifte(BExp, Box<Exp>, Box<Exp>),
    Test(BExp),
    While(BExp, Box<Exp>),
}
