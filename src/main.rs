#[derive(Debug)]
pub enum MyErrors {
    AccountDoesNotExist,
    CannotSerializeAccount,
}

fn main() -> Result<(), MyErrors> {
    Ok(())
}
