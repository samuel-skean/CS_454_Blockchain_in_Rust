use super::coin_solution::CoinSolution;
use super::wallet_solution::WalletSolution;
use std::collections::HashSet;
use std::rc::Rc;

struct BlockchainSolution {
    all_wallets: HashSet<Rc<WalletSolution>>,
}

impl BlockchainSolution {
    fn new() -> BlockchainSolution {
        BlockchainSolution {
            all_wallets: HashSet::new(),
        }
    }

    fn create_wallet(&mut self, capacity: i32) -> Rc<WalletSolution> {
        let new_wallet = Rc::new(WalletSolution::new(capacity));
        self.all_wallets.insert(Rc::clone(&new_wallet));
        new_wallet

    }

    fn create_coin(&self, id: i64) -> Rc<CoinSolution> {
        Rc::new(CoinSolution::new(id))
    }

    fn add_coins(&mut self, wallet: &mut WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    fn transfer_coins(&mut self, from: &mut WalletSolution, to: &mut WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    fn pay_rent(&mut self, wallet: &mut WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    fn redeem_coins(&mut self, wallet: &mut WalletSolution, coins: &HashSet<Rc<CoinSolution>>) -> bool {
        unimplemented!()
    }

    fn get_coins(&self) -> HashSet<Rc<CoinSolution>> {
        unimplemented!()
    }
}
