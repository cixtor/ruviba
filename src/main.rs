use std::env::args;
use std::error::Error;

mod accounts;
mod transactions;

use accounts::Accounts;
use transactions::Transaction;
use transactions::TransactionType;
use transactions::Transactions;

#[derive(Debug)]
pub enum MyErrors {
    NotEnoughArguments,
    AccountDoesNotExist,
    TransactionAmountIsNone,
    CannotWithdrawMoreThanBalance,
    TransactionIsAlreadyInDispute,
    DisputeTransactionDoesNotExist,
    CannotDisputeThisTransactionType,
    CannotResolveThisTransactionType,
    CannotChargebackThisTransactionType,
    TransactionIsNotInDispute,
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

fn process_txns(filename: String) -> Result<(), Box<dyn Error>> {
    let mut txns = Transactions::new();
    let mut accounts = Accounts::new();
    let mut reader = csv::Reader::from_path(filename)?;

    for result in reader.deserialize() {
        let txn: Transaction = result?;

        txns.remember(txn.clone());

        let res = match txn.tx_type {
            TransactionType::Deposit => deposit(&mut accounts, txn),
            TransactionType::Withdraw => withdraw(&mut accounts, txn),
            TransactionType::Dispute => dispute(&mut accounts, txn, &mut txns),
            TransactionType::Resolve => resolve(&mut accounts, txn, &mut txns),
            TransactionType::Chargeback => chargeback(&mut accounts, txn, &mut txns),
        };

        if let Err(err) = res {
            eprintln!("{:?} w/ {:?}", err, txns.last());
        }
    }

    if let Err(err) = accounts.print() {
        println!("{:?}", err);
    }

    Ok(())
}

fn deposit(accounts: &mut Accounts, txn: Transaction) -> Result<(), MyErrors> {
    let amount = txn.read_amount()?;
    accounts.deposit(txn.client, amount);
    Ok(())
}

fn withdraw(accounts: &mut Accounts, txn: Transaction) -> Result<(), MyErrors> {
    let amount = txn.read_amount()?;
    let account = accounts.find(txn.client)?;

    if amount > account.available {
        return Err(MyErrors::CannotWithdrawMoreThanBalance);
    }

    account.available = account.available - amount;
    account.total = account.total - amount;
    Ok(())
}

fn dispute(
    accounts: &mut Accounts,
    txn: Transaction,
    txns: &mut Transactions,
) -> Result<(), MyErrors> {
    let target = txns.find(txn.tx_id)?;
    let amount = target.read_amount()?;

    if target.in_dispute {
        return Err(MyErrors::TransactionIsAlreadyInDispute);
    }

    let account = accounts.find_in_dispute(txn.client, target.client)?;

    match target.tx_type {
        TransactionType::Deposit => {
            account.available = account.available - amount;
            account.held = account.held + amount;
        }
        TransactionType::Withdraw => {
            account.held = account.held + amount;
            account.total = account.total + amount;
        }
        _ => return Err(MyErrors::CannotDisputeThisTransactionType),
    }

    target.in_dispute = true;

    Ok(())
}

fn resolve(
    accounts: &mut Accounts,
    txn: Transaction,
    txns: &mut Transactions,
) -> Result<(), MyErrors> {
    let target = txns.find(txn.tx_id)?;
    let amount = target.read_amount()?;

    if !target.in_dispute {
        return Err(MyErrors::TransactionIsNotInDispute);
    }

    let account = accounts.find_in_dispute(txn.client, target.client)?;

    match target.tx_type {
        TransactionType::Deposit => {
            account.available = account.available + amount;
            account.held = account.held - amount;
        }
        TransactionType::Withdraw => {
            account.held = account.held - amount;
            account.available = account.available + amount;
        }
        _ => return Err(MyErrors::CannotResolveThisTransactionType),
    };

    target.in_dispute = false;

    Ok(())
}

fn chargeback(
    accounts: &mut Accounts,
    txn: Transaction,
    txns: &mut Transactions,
) -> Result<(), MyErrors> {
    let target = txns.find(txn.tx_id)?;
    let amount = target.read_amount()?;

    if !target.in_dispute {
        return Err(MyErrors::TransactionIsNotInDispute);
    }

    let account = accounts.find_in_dispute(txn.client, target.client)?;

    match target.tx_type {
        TransactionType::Deposit => {
            account.held = account.held - amount;
            account.total = account.total - amount;
            account.locked = true;
        }
        TransactionType::Withdraw => {
            account.held = account.held - amount;
            account.total = account.total - amount;
            account.locked = true;
        }
        _ => return Err(MyErrors::CannotChargebackThisTransactionType),
    };

    target.in_dispute = false;

    Ok(())
}
