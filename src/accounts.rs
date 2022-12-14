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

    pub fn create(&mut self, client_id: u16, amount: f64) {
        self.accounts.push(Account {
            client: client_id,
            available: amount,
            held: f64::default(),
            total: amount,
            locked: false,
        });
    }

    pub fn find(&mut self, client_id: u16) -> Result<&mut Account, MyErrors> {
        let account = match self.accounts.iter_mut().find(|x| x.client == client_id) {
            Some(value) => value,
            None => return Err(MyErrors::AccountDoesNotExist),
        };
        Ok(account)
    }

    pub fn find_in_dispute(
        &mut self,
        client_id: u16,
        client_id_in_dispute: u16,
    ) -> Result<&mut Account, MyErrors> {
        let account = match self
            .accounts
            .iter_mut()
            .find(|x| x.client == client_id && x.client == client_id_in_dispute)
        {
            Some(value) => value,
            None => return Err(MyErrors::AccountDoesNotExist),
        };
        Ok(account)
    }

    pub fn deposit(&mut self, client_id: u16, amount: f64) {
        match self.accounts.iter_mut().find(|x| x.client == client_id) {
            Some(acc) => {
                // Account already exists; modify its balance.
                acc.available = acc.available + amount;
                acc.total = acc.total + amount;
            }
            None => {
                // Account does not exist; create a new one.
                self.create(client_id, amount);
            }
        };
    }

    pub fn print(&self) -> Result<(), MyErrors> {
        let mut w = Writer::from_writer(std::io::stdout());
        for item in self.accounts.as_slice() {
            if let Err(_) = w.serialize(item) {
                return Err(MyErrors::CannotSerializeAccount);
            }
        }
        Ok(())
    }
}
