use std::collections::HashMap;

enum AccountingError {
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}

enum TransactTx {
    Deposit(String, u64),
    Withdraw(String, u64),
}

struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    fn new(accounts: HashMap<String, u64>) -> Self { Self { accounts } }

    fn check(header: &[&'static str]) -> std::result::Result<(), AccountingError> {
        match header.get(0) {
            None => Err(AccountingError::AccountNotFound("".to_string())),
            Some(_) => Ok(()),
        }
    }

    fn deposit(
        account_name: String,
        amount: u64,
    ) -> std::result::Result<TransactTx, AccountingError> {
        let depos = TransactTx::Deposit(account_name, amount);
        return Ok(depos);
    }

    fn withdraw(
        account_name: String,
        amount: u64,
    ) -> std::result::Result<TransactTx, AccountingError> {
        let withd = TransactTx::Withdraw(account_name, amount);
        return Ok(withd);
    }

    fn send() -> std::result::Result<(), AccountingError> {
        return Ok(());
    }
}
