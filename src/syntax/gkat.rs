use super::*;
use ahash::HashMap;
use hashconsing::HConsign;

pub struct Gkat<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    // bexp states
    pub(super) name_stamp: u64,
    pub(super) name_map: HashMap<String, VariableIndex>,
    pub(super) bexp_builder: Builder,
    // exp states
    pub(super) exp_hcons: HConsign<Exp_<BExp<A, M>>>,
}

impl<A, M, Builder> Gkat<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn new(builder: Builder) -> Self {
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
