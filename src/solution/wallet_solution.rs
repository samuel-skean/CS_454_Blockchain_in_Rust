use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use super::coin_solution::CoinSolution;


#[derive(PartialEq, Eq, Debug)]
pub struct WalletSolution {
    id: usize,
    pub capacity: usize,
    pub contents: RefCell<HashSet<Rc<CoinSolution>>>
}

impl Hash for WalletSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl WalletSolution {
    pub fn new(id: usize, capacity: usize) -> WalletSolution {
        WalletSolution {
            id,
            capacity,
            contents: RefCell::new(HashSet::new()),
        }
    }
}
