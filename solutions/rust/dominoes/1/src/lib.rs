use std::cell::Cell;

#[derive(Clone)]
struct Domino {
    left_value: Cell<u8>,
    rigth_value: Cell<u8>,
}

impl From<&(u8, u8)> for Domino {
    fn from(value: &(u8, u8)) -> Self {
        Domino { left_value: Cell::new(value.0), rigth_value: Cell::new(value.1) }
    }
}

impl From<Domino> for (u8, u8) {
    fn from(domino: Domino) -> Self {
        (domino.left_value.get(), domino.rigth_value.get())
    }
}

impl Domino {

    fn reverse(&self) {
        let tmp = self.left_value.get();
        self.left_value.set(self.rigth_value.get());
        self.rigth_value.set(tmp);
    }

}

struct Board {
    dominos: Vec<Domino>,
}

impl Board {

    fn new(input: &[(u8, u8)]) -> Self {
        let dominos = input.iter().map(|d| d.into()).collect();
        Board { dominos }
    }

    fn calc_chain(&self, chain: &mut Vec<usize>) -> Option<Vec<(u8, u8)>> {
        if chain.len() == self.dominos.len() {
            return self.generate_result_chain(chain);
        }
        for i in 0..self.dominos.len() {
            if self.try_use(i, chain) {
                let r = self.calc_chain(chain);
                if r.is_none() || !self.first_same_last(&chain) {
                    chain.pop();
                } else {
                    return r;
                }
            }
        }
        None
    }

    fn try_use(&self, i: usize, chain: &mut Vec<usize>) -> bool {
        if !chain.contains(&i) {
            if chain.is_empty() {
                chain.push(i);
                return true;
            } else {
                let last = &self.dominos[chain[chain.len()-1]];
                let current = &self.dominos[i];
                if last.rigth_value == current.rigth_value {
                    current.reverse();
                }
                if last.rigth_value == current.left_value {
                    chain.push(i);
                    return true;
                }
            }
        }
        false
    }
    
    fn generate_result_chain(&self, chain: &[usize]) -> Option<Vec<(u8, u8)>> {
        let mut res = Vec::new();
        for i in chain {
            res.push(self.dominos[*i].clone().into());
        }
        Some(res)
    }
    
    fn first_same_last(&self, chain: &[usize]) -> bool {
        chain.is_empty() || self.dominos[chain[0]].left_value ==
            self.dominos[chain[chain.len()-1]].rigth_value
    }
    
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let board = Board::new(input);
    let mut chain_idx = Vec::new();
    let res = board.calc_chain(&mut chain_idx);
    res
}
