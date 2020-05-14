use std::collections::HashMap;
use std::error::Error;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{Base, send_to};

pub struct Component {
    pub prefix: &'static str
}

impl Component {
    pub fn new() -> Component {
        Component {
            prefix: "cp"
        }
    }

    fn chat(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "sm" => self.send_message(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }

    fn send_message(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let send_msg = data.get("msg").ok_or("err")?.get_object()?;
        let sid = send_msg.get("sid").ok_or("err")?.get_string()?;
        if sid != client.uid {
            println!("{} tried to fake its uid", &client.uid);
            return Ok(())
        }
        let room_name = data.get("rid").ok_or("err")?.get_string()?;
        let player_data = client.player_data.lock().unwrap();
        let mut v = Vec::new();
        v.push(Value::String("cp.cht.sm".to_owned()));
        v.push(msg[2].clone());
        for player_uid in player_data.keys() {
            if player_uid == &client.uid {
                continue;
            }
            let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
            if player.room == room_name {
                send_to(&player.stream, &v, 34)?;
            }
        }
        Ok(())
    }

    fn moderation(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "ar" => self.access_request(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }

    fn access_request(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let pvlg = data.get("pvlg").ok_or("err")?;
        let mut out_data = HashMap::new();
        out_data.insert("pvlg".to_owned(), pvlg.clone());
        out_data.insert("sccss".to_owned(), Value::Boolean(true));
        let mut v = Vec::new();
        v.push(Value::String("cp.m.ar".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }
}

impl Base for Component {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>>{
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "cht" => self.chat(client, msg)?,
            "m" => self.moderation(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
