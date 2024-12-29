use super::*;

impl<A, M, Builder> Solver<A, M, Builder>
where
    A: NodeAddress,
    M: Multiplicity,
    Builder: DecisionDiagramFactory<A, M>,
{
    pub fn reject(
        &mut self,
        gkat: &mut Gkat<A, M, Builder>,
        st: u64,
        m: &Automaton<BExp<A, M>>,
    ) -> BExp<A, M> {
        let eps = m.eps_hat.get(&st).unwrap();
        let mut result = gkat.mk_not(*eps);
        for (b, _, _) in m.delta_hat.get(&st).unwrap() {
            let nb = gkat.mk_not(*b);
            result = gkat.mk_and(result, nb)
        }
        result
    }

    pub fn is_dead(&mut self, st: u64, m: &Automaton<BExp<A, M>>) -> bool {
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
        true
    }
}
