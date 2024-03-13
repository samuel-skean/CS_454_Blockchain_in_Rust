use std::{collections::BTreeSet, rc::Rc};

use super::coin_solution::CoinSolution;


#[derive(PartialEq, Eq, Hash)]
pub struct WalletSolution {
    capacity: i32,
    contents: BTreeSet<Rc<CoinSolution>>
}

impl WalletSolution {
    pub fn new(capacity: i32) -> WalletSolution {
        WalletSolution {
            capacity,
            contents: BTreeSet::new(),
        }
    }
}
