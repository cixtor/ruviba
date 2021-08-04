use crate::MyErrors;
use csv::Writer;
use serde::Serialize;

#[derive(Debug)]
pub struct Accounts {
    accounts: Vec<Account>,
}

#[derive(Debug, Serialize)]
pub struct Account {
    client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Accounts {
    pub fn new() -> Accounts {
        return Accounts {
            accounts: Vec::new(),
        };
    }
}
