pub mod shared {
    pub mod data {
        use crate::models::tx_accounts_user::{TransactTx, AccountingError};
        use crate::shared::data::accounts_user_sqli::shared::data::AccountData;

        pub struct MatchingEngine {}

        pub struct TradingMiddleware {
          matching_engine: MatchingEngine,
          accounts: AccountData,
          transaction_log: Vec<TransactTx>
        }

        impl TradingMiddleware {
          pub fn orderbook(&self) -> Vec<PartialOrd> {}

          pub fn balance_of(&self, signer: &str) -> Result<&u64, AccountingError> {}

          pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<TransactTx, AccountingError> {}

          pub fn send(&mut self, sender: &str, recipient: &str, amount: u64) -> Result<(TransactTx, TransactTx), AccountingError> {}

          pub fn order(&mut self, order: MatchingEngine) -> Result<TransactTx, AccountingError> {}
        }
    } // pub mod data
} // pub mod shared
