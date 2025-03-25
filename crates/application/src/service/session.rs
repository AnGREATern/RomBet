use domain::value_object::Amount;

use crate::usecase::session::{ISessionService, Restart};

pub struct SessionService {
    balance: Amount
}

impl Restart for SessionService {
    fn restart(&self) {
        todo!()
    }
}

impl ISessionService for SessionService {}
