use super::coin_solution::{CoinSolution, CoinStatus};
use super::wallet_solution::WalletSolution;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct BlockchainSolution {
    all_wallets: HashSet<Rc<WalletSolution>>,
    wallet_counter: usize,
}

impl BlockchainSolution {
    pub fn new() -> BlockchainSolution {
        BlockchainSolution {
            all_wallets: HashSet::new(),
            wallet_counter: 0,
        }
    }

    pub fn create_wallet(&mut self, capacity: usize) -> Rc<WalletSolution> {
        let new_wallet = Rc::new(WalletSolution::new(self.wallet_counter, capacity));
        self.wallet_counter += 1;
        self.all_wallets.insert(Rc::clone(&new_wallet));
        new_wallet

    }

    pub fn create_coin(&self, id: i64) -> Rc<CoinSolution> {
        Rc::new(CoinSolution::new(id))
    }

    pub fn add_coins(&mut self, wallet: &WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        let room_for_more_coins = wallet.capacity - wallet.contents.borrow().len();
        if room_for_more_coins >= coins.len() && coins.iter().all(|c| {c.status.get() == CoinStatus::Mined}) {
            for c in coins.iter() {
                c.status.set(CoinStatus::InCirculation);
            }
            wallet.contents.borrow_mut().extend(coins.iter().map(|c| {Rc::clone(c)}));
            true
        } else {
            false
        }
    }

    pub fn transfer_coins(&mut self, from: &WalletSolution, to: &WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    pub fn pay_rent(&mut self, wallet: &WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    pub fn redeem_coins(&mut self, wallet: &WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    pub fn get_coins(&self) -> HashSet<Rc<CoinSolution>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::BlockchainSolution;

    #[test]
    fn basics() {
        let mut b = BlockchainSolution::new();

        let small_wallet = b.create_wallet(2);
        let big_wallet = b.create_wallet(3);

        let coins1 = HashSet::from([b.create_coin(3), b.create_coin(4)]);

        let coins2 = HashSet::from([b.create_coin(0), b.create_coin(1), b.create_coin(2)]);

        assert!(b.add_coins(small_wallet.as_ref(), &coins1));

        // Can't re-add coins once they've been added to anything:
        assert!(!b.add_coins(small_wallet.as_ref(), &coins1));
        assert!(!b.add_coins(big_wallet.as_ref(), &coins1));

        // Can't add too many coins.
        assert!(!b.add_coins(small_wallet.as_ref(), &coins2));
        assert!(b.add_coins(big_wallet.as_ref(), &coins2));

    }
}