use super::*;
use core::fmt;
use std::{collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

// Pure Rust BDD implementation
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BDDNode {
    Leaf(bool),
    Node {
        var: String,
        low: Rc<BDDNode>,
        high: Rc<BDDNode>,
    },
}

impl Debug for BDDNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BDDNode::Leaf(val) => write!(f, "{}", val),
            BDDNode::Node { var, low, high } => {
                write!(f, "{}?{:?}:{:?}", var, low, high)
            }
        }
    }
}

// BExp based on pure Rust BDD.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PureBDDBExp {
    node: Rc<BDDNode>,
}

impl Debug for PureBDDBExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

impl BExp for PureBDDBExp {}

// Gkat based on pure Rust BDD.
pub struct PureBDDGkat {
    name_map: HashMap<String, PureBDDBExp>,
    // Cache for operations
    and_cache: HashMap<(Rc<BDDNode>, Rc<BDDNode>), Rc<BDDNode>>,
    or_cache: HashMap<(Rc<BDDNode>, Rc<BDDNode>), Rc<BDDNode>>,
    not_cache: HashMap<Rc<BDDNode>, Rc<BDDNode>>,
}

impl PureBDDGkat {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::default(),
            and_cache: HashMap::default(),
            or_cache: HashMap::default(),
            not_cache: HashMap::default(),
        }
    }

    fn apply_and(&mut self, a: &Rc<BDDNode>, b: &Rc<BDDNode>) -> Rc<BDDNode> {
        // Check cache
        if let Some(result) = self.and_cache.get(&(a.clone(), b.clone())) {
            return result.clone();
        }

        let result = match (a.as_ref(), b.as_ref()) {
            (BDDNode::Leaf(false), _) | (_, BDDNode::Leaf(false)) => Rc::new(BDDNode::Leaf(false)),
            (BDDNode::Leaf(true), _) => b.clone(),
            (_, BDDNode::Leaf(true)) => a.clone(),
            (
                BDDNode::Node {
                    var: var_a,
                    low: low_a,
                    high: high_a,
                },
                BDDNode::Node {
                    var: var_b,
                    low: low_b,
                    high: high_b,
                },
            ) => {
                if var_a == var_b {
                    // Same variable, recurse on both branches
                    let low = self.apply_and(low_a, low_b);
                    let high = self.apply_and(high_a, high_b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_a.clone(),
                            low,
                            high,
                        })
                    }
                } else if var_a < var_b {
                    // var_a comes before var_b in ordering
                    let low = self.apply_and(low_a, b);
                    let high = self.apply_and(high_a, b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_a.clone(),
                            low,
                            high,
                        })
                    }
                } else {
                    // var_b comes before var_a in ordering
                    let low = self.apply_and(a, low_b);
                    let high = self.apply_and(a, high_b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_b.clone(),
                            low,
                            high,
                        })
                    }
                }
            }
        };

        // Cache the result
        self.and_cache.insert((a.clone(), b.clone()), result.clone());
        result
    }

    fn apply_or(&mut self, a: &Rc<BDDNode>, b: &Rc<BDDNode>) -> Rc<BDDNode> {
        // Check cache
        if let Some(result) = self.or_cache.get(&(a.clone(), b.clone())) {
            return result.clone();
        }

        let result = match (a.as_ref(), b.as_ref()) {
            (BDDNode::Leaf(true), _) | (_, BDDNode::Leaf(true)) => Rc::new(BDDNode::Leaf(true)),
            (BDDNode::Leaf(false), _) => b.clone(),
            (_, BDDNode::Leaf(false)) => a.clone(),
            (
                BDDNode::Node {
                    var: var_a,
                    low: low_a,
                    high: high_a,
                },
                BDDNode::Node {
                    var: var_b,
                    low: low_b,
                    high: high_b,
                },
            ) => {
                if var_a == var_b {
                    // Same variable, recurse on both branches
                    let low = self.apply_or(low_a, low_b);
                    let high = self.apply_or(high_a, high_b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_a.clone(),
                            low,
                            high,
                        })
                    }
                } else if var_a < var_b {
                    // var_a comes before var_b in ordering
                    let low = self.apply_or(low_a, b);
                    let high = self.apply_or(high_a, b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_a.clone(),
                            low,
                            high,
                        })
                    }
                } else {
                    // var_b comes before var_a in ordering
                    let low = self.apply_or(a, low_b);
                    let high = self.apply_or(a, high_b);

                    // Reduce if both branches are the same
                    if low == high {
                        low
                    } else {
                        Rc::new(BDDNode::Node {
                            var: var_b.clone(),
                            low,
                            high,
                        })
                    }
                }
            }
        };

        // Cache the result
        self.or_cache.insert((a.clone(), b.clone()), result.clone());
        result
    }

    fn apply_not(&mut self, a: &Rc<BDDNode>) -> Rc<BDDNode> {
        // Check cache
        if let Some(result) = self.not_cache.get(a) {
            return result.clone();
        }

        let result = match a.as_ref() {
            BDDNode::Leaf(val) => Rc::new(BDDNode::Leaf(!val)),
            BDDNode::Node { var, low, high } => {
                let not_low = self.apply_not(low);
                let not_high = self.apply_not(high);

                // Reduce if both branches are the same
                if not_low == not_high {
                    not_low
                } else {
                    Rc::new(BDDNode::Node {
                        var: var.clone(),
                        low: not_low,
                        high: not_high,
                    })
                }
            }
        };

        // Cache the result
        self.not_cache.insert(a.clone(), result.clone());
        result
    }

    fn is_equiv_nodes(&self, a: &Rc<BDDNode>, b: &Rc<BDDNode>) -> bool {
        Rc::ptr_eq(a, b) || a == b
    }
}

impl Gkat<PureBDDBExp> for PureBDDGkat {
    fn mk_zero(&mut self) -> PureBDDBExp {
        PureBDDBExp {
            node: Rc::new(BDDNode::Leaf(false)),
        }
    }

    fn mk_one(&mut self) -> PureBDDBExp {
        PureBDDBExp {
            node: Rc::new(BDDNode::Leaf(true)),
        }
    }

    fn mk_var(&mut self, s: String) -> PureBDDBExp {
        if let Some(x) = self.name_map.get(&s) {
            return x.clone();
        }

        let node = Rc::new(BDDNode::Node {
            var: s.clone(),
            low: Rc::new(BDDNode::Leaf(false)),
            high: Rc::new(BDDNode::Leaf(true)),
        });

        let bexp = PureBDDBExp { node };
        self.name_map.insert(s, bexp.clone());
        bexp
    }

    fn mk_and(&mut self, b1: &PureBDDBExp, b2: &PureBDDBExp) -> PureBDDBExp {
        PureBDDBExp {
            node: self.apply_and(&b1.node, &b2.node),
        }
    }

    fn mk_or(&mut self, b1: &PureBDDBExp, b2: &PureBDDBExp) -> PureBDDBExp {
        PureBDDBExp {
            node: self.apply_or(&b1.node, &b2.node),
        }
    }

    fn mk_not(&mut self, b: &PureBDDBExp) -> PureBDDBExp {
        PureBDDBExp {
            node: self.apply_not(&b.node),
        }
    }

    fn is_false(&mut self, b: &PureBDDBExp) -> bool {
        matches!(b.node.as_ref(), BDDNode::Leaf(false))
    }

    fn is_equiv(&mut self, b1: &PureBDDBExp, b2: &PureBDDBExp) -> bool {
        self.is_equiv_nodes(&b1.node, &b2.node)
    }
}