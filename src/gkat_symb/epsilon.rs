use crate::gkat_ast::bexp::*;
use crate::gkat_ast::exp::*;
use hashconsing::HConsign;

pub fn epsilon(fb: &mut HConsign<BExp_>, m: &Exp) -> BExp {
    use Exp_::*;
    match m.get() {
        Act(_) => mk_zero(fb),
        Seq(p1, p2) => {
            let b1 = epsilon(fb, p1);
            let b2 = epsilon(fb, p2);
            mk_and(fb, b1, b2)
        }
        If(b, p1, p2) => {
            let b1 = epsilon(fb, p1);
            let b2 = epsilon(fb, p2);
            let b_b1 = mk_and(fb, b.clone(), b1);
            let nb = mk_not(fb, b.clone());
            let nb_b2 = mk_and(fb, nb, b2);
            mk_or(fb, b_b1, nb_b2)
        }
        Test(b) => b.clone(),
        While(b, _) => mk_not(fb, b.clone()),
    }
}
