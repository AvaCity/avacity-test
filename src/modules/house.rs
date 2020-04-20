use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use crate::client::Client;
use crate::common::Value;
use crate::modules::Base;

pub struct House {
    pub prefix: &'static str,
    online: Arc<Mutex<HashMap<String, Client>>>
}

impl House {
    pub fn new(online: Arc<Mutex<HashMap<String, Client>>>) -> House {
        House {
            prefix: "h",
            online: online
        }
    }
}

impl Base for House {
    fn handle(&self, client: &Client, msg: &Vec<Value>) {
        let tmp = msg[1].get_string().unwrap();
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        println!("command {}", command);
    }
}
