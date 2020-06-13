use std::collections::HashMap;
use std::error::Error;
use crate::modules::{Base, get_location, get_plr};
use crate::common::Value;
use crate::client::Client;

pub struct Player {
    pub prefix: &'static str,
}

impl Player {
    pub fn new() -> Player {
        Player {
            prefix: "pl"
        }
    }

    fn players_by_id(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let uids = data.get("uids").ok_or("err")?.get_vector()?;
        let clid = data.get("clid").ok_or("err")?.get_string()?;
        let mut players = Vec::new();
        let player_data = client.player_data.read().unwrap();
        for tmp in uids {
            let uid = tmp.get_string()?;
            let plr = get_plr(&uid, &player_data, &client.redis)?;
            match plr {
                Some(v) => players.push(Value::Object(v)),
                None => continue
            };
        }
        let mut out_data = HashMap::new();
        out_data.insert("plrs".to_owned(), Value::Vector(players));
        out_data.insert("clid".to_owned(), Value::String(clid));
        let mut v = Vec::new();
        v.push(Value::String("pl.get".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn follow(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let uid = data.get("uid").ok_or("err")?.get_string()?;
        let player_data = client.player_data.read().unwrap();
        let locinfo = get_location(&uid, &player_data);
        let scs: &str;
        match locinfo {
            Value::Object(_) => {
                scs = "success";
            },
            _ => {
                scs = "userOffline";
            }
        }
        let mut out_data = HashMap::new();
        out_data.insert("scs".to_owned(), Value::String(scs.to_owned()));
        out_data.insert("locinfo".to_owned(), locinfo);
        let mut v = Vec::new();
        v.push(Value::String("pl.flw".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }
}

impl Base for Player {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "gid" => self.players_by_id(client, msg)?,
            "flw" => self.follow(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
