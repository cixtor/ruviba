use crate::MyErrors;
use serde::Deserialize;

#[derive(Debug)]
pub struct Transactions {
    txns: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    pub amount: Option<f64>,
    #[serde(default)]
    pub in_dispute: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TransactionType {
    #[serde(alias = "deposit")]
    Deposit,
    #[serde(alias = "withdrawal")]
    Withdraw,
    #[serde(alias = "dispute")]
    Dispute,
    #[serde(alias = "resolve")]
    Resolve,
    #[serde(alias = "chargeback")]
    Chargeback,
}

impl Transactions {
    pub fn new() -> Transactions {
        return Transactions { txns: Vec::new() };
    }

    pub fn remember(&mut self, txn: Transaction) {
        self.txns.push(txn);
    }

    pub fn find(&mut self, txn_id: u32) -> Result<&mut Transaction, MyErrors> {
        let txn = match self.txns.iter_mut().find(|x| x.tx_id == txn_id) {
            Some(value) => value,
            None => return Err(MyErrors::DisputeTransactionDoesNotExist),
        };
        Ok(txn)
    }

    pub fn last(&self) -> &Transaction {
        self.txns.last().expect("empty transaction history")
    }
}
