use super::*;
use std::slice::Iter;

pub struct GuardIterator<'a, B, G> {
    gkat: &'a mut G,
    guard: &'a B,
    iter: Iter<'a, (B, u64, u64)>,
}

impl<'a, B: BExp, G: Gkat<B>> GuardIterator<'a, B, G> {
    pub fn new(gkat: &'a mut G, guard: &'a B, iter: Iter<'a, (B, u64, u64)>) -> Self {
        Self {
            gkat: gkat,
            guard: guard,
            iter: iter,
        }
    }
}

impl<'a, B: BExp, G: Gkat<B>> Iterator for GuardIterator<'a, B, G> {
    type Item = (B, u64, u64);

    #[inline]
    fn next(&mut self) -> Option<(B, u64, u64)> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    let b = self.gkat.mk_and(self.guard, &x.0);
                    if self.gkat.is_false(&b) {
                        continue;
                    } else {
                        return Some((b, x.1, x.2));
                    }
                }
                None => return None,
            }
        }
    }
}
