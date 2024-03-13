use std::{cell::Cell, hash::Hash};

#[derive(Clone, Copy)]
pub enum CoinStatus {
    Mined,
    InCirculation,
    Rent,
    Redeemed,
}

pub struct CoinSolution {
    id: i64,
    status: Cell<CoinStatus>,
}

// Heavily relying on the invariant that no two coins share the same id.
impl PartialEq for CoinSolution {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CoinSolution {}

impl Hash for CoinSolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl CoinSolution {
    pub fn new(id: i64) -> Self {
        CoinSolution {
            id,
            status: Cell::new(CoinStatus::Mined),
        }
    }

    pub fn get_status(&self) -> CoinStatus {
        self.status.get()
    }
}
