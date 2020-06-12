use std::collections::HashMap;
use std::error::Error;
use crate::modules::{Base, get_plr};
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
            players.push(Value::Object(get_plr(&uid, &player_data, &client.redis)?.ok_or("err")?));
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
}

impl Base for Player {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "gid" => self.players_by_id(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
