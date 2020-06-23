use std::collections::HashMap;
use std::error::Error;
use redis::Commands;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{Base, notify};
use crate::parser;


lazy_static! {
    pub static ref TROPHIES: Vec<String> = parser::parse_trophies();
}

pub struct Passport {
    pub prefix: &'static str,
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            prefix: "psp",
        }
    }

    fn passport(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let uid = data.get("uid").ok_or("err")?.get_string()?;
        let mut ach = HashMap::new();
        let mut tr = HashMap::new();
        for trophy in TROPHIES.iter() {
            let mut item = HashMap::new();
            item.insert("trrt".to_owned(), Value::I32(0));
            item.insert("trcd".to_owned(), Value::I32(0));
            item.insert("trid".to_owned(), Value::String(trophy.to_owned()));
            tr.insert(trophy.to_owned(), Value::Object(item));
        }
        ach.insert("ac".to_owned(), Value::Object(HashMap::new()));
        ach.insert("tr".to_owned(), Value::Object(tr));
        let mut psp = HashMap::new();
        psp.insert("uid".to_owned(), Value::String(uid));
        psp.insert("rel".to_owned(), Value::Object(HashMap::new()));
        psp.insert("ach".to_owned(), Value::Object(ach));
        let mut out_data = HashMap::new();
        out_data.insert("psp".to_owned(), Value::Object(psp));
        let mut v = Vec::new();
        v.push(Value::String("psp.psp".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn set_trophy(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let trid = data.get("trid").ok_or("err")?;
        let mut con = client.redis.get_connection()?;
        match trid {
            Value::String(v) => {
                let _: () = con.set(format!("uid:{}:trid", &client.uid), v)?;
            },
            _ => {
                let _: () = con.del(format!("uid:{}:trid", &client.uid))?;
            }
        }
        let mut data = HashMap::new();
        data.insert("trid".to_owned(), trid.clone());
        let mut v = Vec::new();
        v.push(Value::String("psp.sttrph".to_owned()));
        v.push(Value::Object(data));
        client.send(&v, 34)?;
        notify::update_city_info(client)?;
        Ok(())
    }
}

impl Base for Passport {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "psp" => self.passport(client, msg)?,
            "sttrph" => self.set_trophy(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
