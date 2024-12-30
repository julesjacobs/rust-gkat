use super::*;

impl Solver {
    pub fn reject(&mut self, gkat: &mut Gkat, exp: &Exp) -> BExp {
        let dexp = self.derivative(gkat, exp);
        let eps = self.epsilon(gkat, exp);
        dexp.iter().fold(eps.not(), |acc, (b, _, _)| {
            let nb = b.not();
            nb.and(&acc)
        })
    }

    pub fn is_dead(&mut self, gkat: &mut Gkat, exp: &Exp) -> bool {
        let mut stack = Vec::new();
        stack.push(exp.clone());
        self.explored.clear();
        while let Some(exp) = stack.pop() {
            if self.dead_states.contains(&exp) || self.explored.contains(&exp) {
                continue;
            }
            self.explored.insert(exp.clone());
            let eps = self.epsilon(gkat, &exp);
            if eps.is_false() {
                for (_, e, _) in self.derivative(gkat, &exp) {
                    stack.push(e);
                }
            } else {
                return false;
            }
        }
        self.dead_states.extend(self.explored.iter().cloned());
        true
    }
}
