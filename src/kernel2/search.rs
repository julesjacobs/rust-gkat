use super::*;

impl Solver {
    pub fn reject(&mut self, st: u64, m: &Automaton) -> BExp {
        let eps = m.eps_hat.get(&st).unwrap();
        let mut result = eps.not();
        for (b, _, _) in m.delta_hat.get(&st).unwrap() {
            let nb = b.not();
            result = result.and(&nb)
        }
        return result;
    }

    pub fn is_dead(&mut self, st: u64, m: &Automaton) -> bool {
        let mut stack = Vec::new();
        stack.push(st);
        self.explored.clear();
        while let Some(st) = stack.pop() {
            if self.dead_states.contains(&st) || self.explored.contains(&st) {
                continue;
            }
            self.explored.insert(st);
            let eps = m.eps_hat.get(&st).unwrap();
            if eps.is_false() {
                for (_, st, _) in m.delta_hat.get(&st).unwrap() {
                    stack.push(*st);
                }
            } else {
                return false;
            }
        }
        self.dead_states.extend(self.explored.iter());
        return true;
    }
}
