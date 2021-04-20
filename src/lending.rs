use std::collections::BTreeMap;

use crate::{errors::Result, util::get_now_timestamp};
use crate::{client::Client, model::LendingAccount, util::build_request};





#[derive(Clone)]
pub struct Lending {
    pub client: Client,
    pub recv_window: u64,
}


impl Lending {
    pub fn get_account(&self) -> Result<LendingAccount> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("timestamp".into(), get_now_timestamp().unwrap().to_string());
        self.client.get_signed::<LendingAccount, _>("/sapi/v1/lending/union/account", Some(build_request(parameters)))
    }
}