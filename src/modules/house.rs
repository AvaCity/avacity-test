use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{Base, get_appearance};

pub struct House {
    pub prefix: &'static str,
    online: Arc<Mutex<HashMap<String, Client>>>
}

fn apprnc_not_found() -> Vec<Value> {
    let mut v: Vec<Value> = Vec::new();
    v.push(Value::String("h.minfo".to_owned()));
    let mut data: HashMap<String, Value> = HashMap::new();
    data.insert("has.avtr".to_owned(), Value::Boolean(false));
    v.push(Value::Object(data));
    return v;
}

impl House {
    pub fn new(online: Arc<Mutex<HashMap<String, Client>>>) -> House {
        House {
            prefix: "h",
            online: online
        }
    }

    fn get_my_info(&self, client: &Client, msg: &Vec<Value>) {
        let apprnc = get_appearance(&client.uid, &client.redis);
        match apprnc {
            Some(data) => {
                println!("got apprnc")
            }
            None => {
                let msg = apprnc_not_found();
                client.send(msg, 34);
            }
        }
    }
}

impl Base for House {
    fn handle(&self, client: &Client, msg: &Vec<Value>) {
        let tmp = msg[1].get_string().unwrap();
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        println!("command {}", command);
        match command {
            "minfo" => self.get_my_info(client, msg),
            _ => println!("Command {} not found", tmp)
            
        }
    }
}
