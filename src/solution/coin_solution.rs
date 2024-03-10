use std::cell::Cell;

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
