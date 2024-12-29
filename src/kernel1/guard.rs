use crate::syntax::*;
use rsdd::{builder::BottomUpBuilder, repr::DDNNFPtr};
use std::slice::Iter;

pub struct GuardIterator<'a, 'b, BExp: DDNNFPtr<'a>, Builder> {
    gkat: &'b mut Gkat<'a, BExp, Builder>,
    guard: BExp,
    iter: Iter<'b, (BExp, Exp<BExp>, u64)>,
}

impl<'a, 'b, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>>
    GuardIterator<'a, 'b, BExp, Builder>
{
    pub fn new(
        gkat: &'b mut Gkat<'a, BExp, Builder>,
        guard: BExp,
        iter: Iter<'b, (BExp, Exp<BExp>, u64)>,
    ) -> Self {
        GuardIterator {
            gkat: gkat,
            guard: guard,
            iter: iter,
        }
    }
}

impl<'a, 'b, BExp: DDNNFPtr<'a>, Builder: BottomUpBuilder<'a, BExp>> Iterator
    for GuardIterator<'a, 'b, BExp, Builder>
{
    type Item = (BExp, Exp<BExp>, u64);
    #[inline]
    fn next(&mut self) -> Option<(BExp, Exp<BExp>, u64)> {
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
