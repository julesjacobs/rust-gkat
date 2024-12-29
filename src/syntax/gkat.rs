use super::*;
use ahash::HashMap;
use hashconsing::HConsign;
use rsdd::repr::{DDNNFPtr, VarLabel};

pub struct Gkat<'a, BExp: DDNNFPtr<'a>, Builder> {
    // bexp states
    pub(super) name_stamp: u64,
    pub(super) name_map: HashMap<String, VarLabel>,
    pub(super) bexp_builder: &'a Builder,
    // exp states
    pub(super) exp_hcons: HConsign<Exp_<BExp>>,
}

impl<'a, BExp: DDNNFPtr<'a>, Builder> Gkat<'a, BExp, Builder> {
    pub fn new(builder: &'a Builder) -> Self {
        Gkat {
            // bexp init
            name_stamp: 0,
            name_map: HashMap::default(),
            bexp_builder: builder,
            // exp init
            exp_hcons: HConsign::empty(),
        }
    }
}
