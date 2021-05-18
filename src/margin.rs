use crate::client::Client;



// 杠杆
#[derive(Clone)]
pub struct Margin {
    pub client: Client,
    pub recv_window: u64,
}



impl Margin {
    pub fn isolated_account() {
        
    }
}