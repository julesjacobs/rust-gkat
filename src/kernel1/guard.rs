use super::*;
use std::slice::Iter;

pub struct GuardIterator<'a> {
    guard: &'a BExp,
    iter: Iter<'a, (BExp, Exp, u64)>,
}

impl<'a> GuardIterator<'a> {
    pub fn new(guard: &'a BExp, iter: Iter<'a, (BExp, Exp, u64)>) -> Self {
        GuardIterator {
            guard: guard,
            iter: iter,
        }
    }
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = (BExp, Exp, u64);

    #[inline]
    fn next(&mut self) -> Option<(BExp, Exp, u64)> {
        loop {
            match self.iter.next() {
                Some(x) => {
                    let b = self.guard.and(&x.0);
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
