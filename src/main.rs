#[derive(Debug)]
pub enum MyErrors {
    AccountDoesNotExist,
}

fn main() -> Result<(), MyErrors> {
    Ok(())
}
