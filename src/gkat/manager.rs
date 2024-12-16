use super::*;
use ahash::HashMap;
use hashconsing::HConsign;
use rsdd::builder::BottomUpBuilder;
use rsdd::repr::{DDNNFPtr, VarLabel};

pub struct GkatManager<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> {
    // bexp states
    pub(super) name_stamp: u64,
    pub(super) name_map: HashMap<String, VarLabel>,
    pub(super) bexp_builder: &'a Builder,
    // exp states
    pub(super) exp_hcons: HConsign<Exp_<Ptr>>,
}

impl<'a, Ptr: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, Ptr>> GkatManager<'a, Ptr, Builder> {
    pub fn new(builder: &'a Builder) -> Self {
        GkatManager {
            // bexp init
            name_stamp: 0,
            name_map: HashMap::default(),
            bexp_builder: builder,
            // exp init
            exp_hcons: HConsign::empty(),
        }
    }
}
