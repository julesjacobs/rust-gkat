use super::*;
use std::slice::Iter;

pub struct GuardIterator<'a> {
    gkat: &'a mut Gkat,
    guard: &'a BExp,
    iter: Iter<'a, (BExp, u64, u64)>,
}

impl<'a> GuardIterator<'a> {
    pub fn new(gkat: &'a mut Gkat, guard: &'a BExp, iter: Iter<'a, (BExp, u64, u64)>) -> Self {
        GuardIterator {
            gkat: gkat,
            guard: guard,
            iter: iter,
        }
    }
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = (BExp, u64, u64);

    #[inline]
    fn next(&mut self) -> Option<(BExp, u64, u64)> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    let b = self.guard.and(&x.0);
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
