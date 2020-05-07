use std::collections::HashMap;
use std::error::Error;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{get_plr, Base, location};

static PLAYER_LIMIT: &'static u8 = &15;

pub struct Outside {
    pub prefix: &'static str
}

impl Outside {
    pub fn new() -> Outside {
        Outside {
            prefix: "o"
        }
    }

    fn get_room(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let lid = data.get("lid").ok_or("err")?.get_string()?;
        let gid = data.get("gid").ok_or("err")?.get_string()?;
        let mut num: i32;
        let mut room_name: String;
        if data.contains_key("rid") {
            num = data.get("rid").unwrap().get_string()?.parse::<i32>()?;
            room_name = format!("{}_{}_{}", lid, gid, num);
        }
        else {
            let player_data = client.player_data.lock().unwrap();
            let mut players_count = 0;
            num = 1;
            loop {
                room_name = format!("{}_{}_{}", lid, gid, num);
                for player_uid in player_data.keys() {
                    let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
                    if player.room == room_name {
                        players_count = players_count + 1;
                    } 
                }
                if &players_count <= PLAYER_LIMIT {
                    break;
                }
                num = num + 1;
            }
        }
        location::leave_room(self.prefix, client)?;
        location::join_room(self.prefix, client, &room_name)?;
        let mut out_data = HashMap::new();
        out_data.insert("rid".to_owned(), Value::String(room_name));
        let mut v = Vec::new();
        v.push(Value::String("o.gr".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn room(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "info" => self.room_info(client, msg)?,
            _ => location::room(client, msg)?
        }
        Ok(())
    }

    fn room_info(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let room_name = msg[0].get_string()?;
        let mut rmmb = Vec::new();
        let player_data = client.player_data.lock().unwrap();
        for player_uid in player_data.keys() {
            let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
            if player.room == room_name {
                match get_plr(&player_uid, &player_data, &client.redis)? {
                    Some(plr) => rmmb.push(Value::Object(plr)),
                    None => continue
                }
            }
        }
        let mut out_data = HashMap::new();
        out_data.insert("rmmb".to_owned(), Value::Vector(rmmb));
        out_data.insert("evn".to_owned(), Value::None);
        let mut v = Vec::new();
        v.push(Value::String("o.r.info".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }
}

impl Base for Outside {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "gr" => self.get_room(client, msg)?,
            "r" => self.room(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
