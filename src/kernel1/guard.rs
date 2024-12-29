use super::*;
use std::slice::Iter;

pub struct GuardIterator<'a, A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    gkat: &'a mut Gkat<A, M, Builder>,
    guard: BExp<A, M>,
    iter: Iter<'a, (BExp<A, M>, Exp<BExp<A, M>>, u64)>,
}

impl<'a, A, M, Builder> GuardIterator<'a, A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn new(
        gkat: &'a mut Gkat<A, M, Builder>,
        guard: BExp<A, M>,
        iter: Iter<'a, (BExp<A, M>, Exp<BExp<A, M>>, u64)>,
    ) -> Self {
        GuardIterator {
            gkat: gkat,
            guard: guard,
            iter: iter,
        }
    }
}

impl<'a, A, M, Builder> Iterator for GuardIterator<'a, A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    type Item = (BExp<A, M>, Exp<BExp<A, M>>, u64);

    #[inline]
    fn next(&mut self) -> Option<(BExp<A, M>, Exp<BExp<A, M>>, u64)> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    let guard = self.guard;
                    let b = self.gkat.mk_and(guard, x.0);
                    if b.is_false() {
                        continue;
                    } else {
                        return Some((b, x.1.clone(), x.2));
                    }
                }
                None => return None,
            }
        }
    }
}
