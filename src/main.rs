use std::env::args;
use std::error::Error;

#[derive(Debug)]
pub enum MyErrors {
    NotEnoughArguments,
    AccountDoesNotExist,
    TransactionAmountIsNone,
    DisputeTransactionDoesNotExist,
    CannotSerializeAccount,
}

fn main() -> Result<(), MyErrors> {
    let filename = match args().nth(1) {
        Some(value) => value,
        None => return Err(MyErrors::NotEnoughArguments),
    };

    println!("{}", filename);

    Ok(())
}

fn deposit(accounts: &mut Accounts, txn: Transaction) -> Result<(), MyErrors> {
    let amount = txn.read_amount()?;
    accounts.deposit(txn.client, amount);
    Ok(())
}
