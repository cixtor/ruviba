#[derive(Debug)]
pub struct Transactions {
    txns: Vec<Transaction>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
