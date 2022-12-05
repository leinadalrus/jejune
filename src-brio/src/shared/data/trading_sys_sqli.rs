pub mod shared {
    pub mod data {
        use diesel::pg::PgConnection;
        use diesel::prelude::*;
        use dotenvy::dotenv;
        use std::env;

        use crate::models::tx_accounts_user::{TransactTx, AccountingError};
        use crate::shared::data::accounts_user_sqli::shared::data::AccountData;

        pub struct MatchingEng {}

        pub struct TradingPlat {
          matching_engine: MatchingEng,
          accounts: AccountData,
          transaction_log: Vec<TransactTx>
        }

        impl TradingPlat {
          pub fn orderbook(&self) -> Vec<PartialOrd> {}

          pub fn balance_of(&self, signer: &str) -> Result<&u64, AccountingError> {}

          pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<TransactTx, AccountingError> {}

          pub fn send(&mut self, sender: &str, recipient: &str, amount: u64) -> Result<(TransactTx, TransactTx), AccountingError> {}

          pub fn order(&mut self, order: MatchingEng) -> Result<TransactTx, AccountingError> {}
        }
    } // pub mod data
} // pub mod shared
