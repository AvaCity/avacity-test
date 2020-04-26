use std::collections::HashMap;
use std::error::Error;
use redis::Commands;
use crate::common::Value;
use crate::client::Client;
use crate::modules::{Base, notify};


pub struct Billing {
    pub prefix: &'static str
}

impl Billing {
    pub fn new() -> Billing {
        Billing {
            prefix: "b"
        }
    }

    fn check_purchase(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let pack = data.get("prid").ok_or("key not found")?.get_string()?;
        let amount: i32;
        match pack.as_str() {
            "pack10" => amount = 10,
            "pack30" => amount = 32,
            "pack50" => amount = 55,
            "pack100" => amount = 120,
            "pack200" => amount = 260,
            "pack500" => amount = 700,
            "pack1000" => amount = 1450,
            "pack1500" => amount = 2200,
            _ => amount = 0
        }
        let mut con = client.redis.get_connection()?;
        let _: () = con.incr(format!("uid:{}:gld", &client.uid), amount)?;
        notify::update_resources(client)?;
        let mut out_data = HashMap::new();
        out_data.insert("ingld".to_owned(), Value::I32(amount));
        let mut v = Vec::new();
        v.push(Value::String("b.ingld".to_owned()));
        v.push(Value::Object(out_data));
        client.send(v, 34);
        Ok(())
    }
}

impl Base for Billing {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "chkprchs" => self.check_purchase(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
