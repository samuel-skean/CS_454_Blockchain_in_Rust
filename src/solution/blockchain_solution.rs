use super::coin_solution::CoinSolution;
use super::wallet_solution::WalletSolution;
use std::collections::HashSet;

struct BlockchainSolution {
    all_wallets: HashSet<WalletSolution>,
}

impl BlockchainSolution {
    fn attempt_coins_removal(
        &mut self,
        wallet: WalletSolution,
        coins: HashSet<CoinSolution>,
    ) -> bool {
        false
    }
}
