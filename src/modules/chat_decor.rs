use std::collections::HashMap;
use std::error::Error;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{notify, Base};
use crate::inventory;
use crate::parser;
use redis::Commands;

lazy_static! {
    static ref GAME_ITEMS: HashMap<String, parser::Item> = parser::parse_game_items();
}

pub struct ChatDecor {
    pub prefix: &'static str
}

impl ChatDecor {
    pub fn new() -> ChatDecor {
        ChatDecor {
            prefix: "chtdc"
        }
    }

    fn save_chat_decor_model(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let new_bubble = data.get("chtnwbd").ok_or("err")?.get_bool()?;
        let new_text_color = data.get("chtnwtc").ok_or("err")?.get_bool()?;
        let chat_decor = data.get("chtdc").ok_or("err")?.get_object()?;
        let mut con = client.redis.get_connection()?;
        if new_bubble {
            let bubble = chat_decor.get("bdc").ok_or("err")?;
            match bubble {
                Value::String(v) => {
                    if inventory::get_item(&client.redis, &client.uid, v)?.is_none() {
                        if !self.buy_decor(client, v)? {
                            return Ok(())
                        }
                    }
                    let _: () = con.set(format!("uid:{}:bubble", &client.uid), v)?;
                },
                _ => {
                    let _: () = con.del(format!("uid:{}:bubble", &client.uid))?;
                }
            }
        }
        if new_text_color {
            let text_color = chat_decor.get("tcl").ok_or("err")?;
            match text_color {
                Value::String(v) => {
                    if inventory::get_item(&client.redis, &client.uid, v)?.is_none() {
                        if !self.buy_decor(client, v)? {
                            return Ok(())
                        }
                    }
                    let _: () = con.set(format!("uid:{}:text_color", &client.uid), v)?;
                },
                _ => {
                    let _: () = con.del(format!("uid:{}:text_color", &client.uid))?;
                }
            }
        }
        notify::update_chat_decor(client)?;
        let mut v = Vec::new();
        v.push(Value::String("chtdc.schtm".to_owned()));
        v.push(Value::Object(HashMap::new()));
        client.send(&v, 34)?;
        Ok(())
    }

    fn buy_decor(&self, client: &Client, tpid: &str) -> Result<bool, Box<dyn Error>> {
        if !GAME_ITEMS.contains_key(tpid) {
            return Ok(false)
        }
        let item = GAME_ITEMS.get(tpid).unwrap();
        let mut con = client.redis.get_connection()?;
        let have_gold: i32 = con.get(format!("uid:{}:gld", &client.uid))?;
        let have_silver: i32 = con.get(format!("uid:{}:slvr", &client.uid))?;
        if item.gold > have_gold || item.silver > have_silver {
            return Ok(false)
        }
        let _: () = con.incr(format!("uid:{}:gld", &client.uid), -item.gold)?;
        let _: () = con.incr(format!("uid:{}:slvr", &client.uid), -item.silver)?;
        inventory::add_item(&client.redis, &client.uid, &tpid, "gm", 1)?;
        notify::update_item(client, &tpid)?;
        notify::update_resources(client)?;
        Ok(true)
    }
}

impl Base for ChatDecor {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "schtm" => self.save_chat_decor_model(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
