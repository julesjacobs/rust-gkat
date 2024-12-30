use super::*;
use ahash::HashMap;
use cudd::{Cudd_Init, CUDD_CACHE_SLOTS, CUDD_UNIQUE_SLOTS};
use hashconsing::HConsign;

pub(super) struct Builder(pub *mut DdManager);

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { Cudd_Quit(self.0) };
    }
}

pub struct Gkat {
    pub(super) name_map: HashMap<String, BExp>,
    pub(super) exp_hcons: HConsign<Exp_>,
    pub(super) bexp_builder: Builder,
}

impl Gkat {
    pub fn new() -> Self {
        let builder = unsafe { Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0) };
        Gkat {
            name_map: HashMap::default(),
            exp_hcons: HConsign::empty(),
            bexp_builder: Builder(builder),
        }
    }
}
