#[derive(Debug)]
pub enum MyErrors {
    AccountDoesNotExist,
    DisputeTransactionDoesNotExist,
    CannotSerializeAccount,
}

fn main() -> Result<(), MyErrors> {
    Ok(())
}
