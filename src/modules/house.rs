use std::collections::{HashMap, HashSet};
use redis::Commands;
use crate::client::Client;
use crate::common::Value;
use crate::inventory;
use crate::modules::{Base, get_plr};

pub struct House {
    pub prefix: &'static str,
}

pub fn get_rooms(uid: &String, redis: &redis::Client) -> Vec<Value> {
    let mut con = redis.get_connection().unwrap();
    let rooms: HashSet<String> = con.smembers(format!("rooms:{}", uid)).unwrap();
    let mut out = Vec::new();
    for room in rooms {
        let mut out_room = HashMap::new();
        let data: Vec<String> = con.lrange(format!("rooms:{}:{}", uid, &room), 0, -1).unwrap();
        let items = get_room_items(uid, &room, redis);
        out_room.insert("f".to_owned(), Value::Vector(items));
        out_room.insert("w".to_owned(), Value::I32(13));
        out_room.insert("id".to_owned(), Value::String(room.clone()));
        out_room.insert("lev".to_owned(), Value::I32(data[1].parse::<i32>().unwrap()));
        out_room.insert("l".to_owned(), Value::I32(13));
        out_room.insert("nm".to_owned(), Value::String(data[0].clone()));
        out.push(Value::Object(out_room));
    }
    return out;
}

pub fn get_room_items(uid: &String, room: &String, redis: &redis::Client) -> Vec<Value> {
    let mut con = redis.get_connection().unwrap();
    let items: HashSet<String> = con.smembers(format!("rooms:{}:{}:items", uid, room)).unwrap();
    let mut out = Vec::new();
    for item in items {
        let data: HashMap<String, String> = con.hgetall(format!("rooms:{}:{}:items:{}", uid, room, &item)).unwrap();
        let mut out_item: HashMap<String, Value> = HashMap::new();
        let splitted: Vec<&str> = item.split("_").collect();
        out_item.insert("tpid".to_owned(), Value::String(splitted[0].to_owned()));
        out_item.insert("lid".to_owned(), Value::I32(splitted[1].parse::<i32>().unwrap()));
        for key in data.keys() {
            out_item.insert(key.clone(), Value::String(data.get(key).unwrap().clone()));
        }
        out.push(Value::Object(out_item));
    }
    return out;
}

impl House {
    pub fn new() -> House {
        House {
            prefix: "h"
        }
    }

    fn get_my_info(&self, client: &Client, _msg: &Vec<Value>) {
        let mut v: Vec<Value> = Vec::new();
        v.push(Value::String("h.minfo".to_owned()));
        let mut data: HashMap<String, Value> = HashMap::new();
        match get_plr(&client.uid, &client.redis) {
            Some(mut plr) => {
                let mut con = client.redis.get_connection().unwrap();
                let silver: i32 = con.get(format!("uid:{}:slvr", &client.uid)).unwrap_or(0);
                let gold: i32 = con.get(format!("uid:{}:gld", &client.uid)).unwrap_or(0);
                let energy: i32 = con.get(format!("uid:{}:enrg", &client.uid)).unwrap_or(0);
                let mut res = HashMap::new();
                res.insert("slvr".to_owned(), Value::I32(silver));
                res.insert("gld".to_owned(), Value::I32(gold));
                res.insert("enrg".to_owned(), Value::I32(energy));
                res.insert("emd".to_owned(), Value::I32(0));
                plr.insert("res".to_owned(), Value::Object(res));
                let mut hs = HashMap::new();
                hs.insert("r".to_owned(), Value::Vector(get_rooms(&client.uid, &client.redis)));
                hs.insert("lt".to_owned(), Value::I32(0));
                plr.insert("hs".to_owned(), Value::Object(hs));
                plr.insert("inv".to_owned(), Value::Object(inventory::get(&client.uid, &client.redis)));
                data.insert("plr".to_owned(), Value::Object(plr));
                data.insert("tm".to_owned(), Value::I32(1));
            }
            None => {
                data.insert("has.avtr".to_owned(), Value::Boolean(false));
            }
        }
        v.push(Value::Object(data));
        client.send(v, 34);
    }
}

impl Base for House {
    fn handle(&self, client: &Client, msg: &Vec<Value>) {
        let tmp = msg[1].get_string().unwrap();
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "minfo" => self.get_my_info(client, msg),
            "gr" => self.get_room(client, msg),
            _ => println!("Command {} not found", tmp)
            
        }
    }
}
