#[derive(Debug)]
pub enum MyErrors {
    AccountDoesNotExist,
    TransactionAmountIsNone,
    DisputeTransactionDoesNotExist,
    CannotSerializeAccount,
}

fn main() -> Result<(), MyErrors> {
    Ok(())
}
