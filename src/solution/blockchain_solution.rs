use super::coin_solution::CoinSolution;
use super::wallet_solution::WalletSolution;
use std::collections::HashSet;
use std::rc::Rc;

struct BlockchainSolution {
    all_wallets: HashSet<WalletSolution>,
}

impl BlockchainSolution {
    fn new() -> BlockchainSolution {
        unimplemented!()
    }

    fn create_wallet(capacity: i32) -> Rc<WalletSolution> {
        unimplemented!()
    }

    fn create_coin(id: i32) -> Rc<CoinSolution> {
        unimplemented!()
    }

    fn add_coins(wallet: &mut WalletSolution, coins: &HashSet<CoinSolution>) -> bool {
        unimplemented!()
    }

    fn transfer_coins(from: &mut WalletSolution, to: &mut WalletSolution, coins: &HashSet<CoinSolution>) -> bool {
        unimplemented!()
    }

    fn pay_rent(wallet: &mut WalletSolution, coins: &HashSet<CoinSolution>) -> bool {
        unimplemented!()
    }

    fn redeem_coins(wallet: &mut WalletSolution, coins: &HashSet<CoinSolution>) -> bool {
        unimplemented!()
    }

    fn get_coins() -> HashSet<CoinSolution> {
        unimplemented!()
    }
}
