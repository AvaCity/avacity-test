use std::collections::{HashMap, HashSet};
use std::error::Error;
use redis::Commands;
use crate::inventory;
use crate::parser;
use crate::client::Client;
use crate::common::Value;
use crate::modules::{Base, get_city_info, house, notify};

lazy_static! {
    static ref ITEMS: HashMap<String, parser::Item> = parser::parse_furniture();
}

pub struct Furniture {
    pub prefix: &'static str,
}

impl Furniture {
    pub fn new() -> Furniture {
        Furniture {
            prefix: "frn",
        }
    }

    fn save(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[0].get_string()?;
        let splitted: Vec<&str> = tmp.split("_").collect();
        let room = splitted[2];
        let data = msg[2].get_object()?;
        let items = data.get("f").ok_or("err")?.get_vector()?;
        for tmp1 in items {
            let item = tmp1.get_object()?;
            let type_ = item.get("t").ok_or("err")?.get_i32()?;
            match type_ {
                0 => self.type_add(item, room, client)?,
                1 => self.type_update(item, room, client)?,
                2 => self.type_remove(item, room, client)?,
                //3 => self.type_replace_door(item, room, client),
                //4 => self.type_change_color(item, room, client),
                _ => continue
            }
        }
        let mut out_data = HashMap::new();
        out_data.insert("inv".to_owned(), Value::Object(inventory::get(&client.uid, &client.redis)?));
        out_data.insert("ci".to_owned(), Value::Object(get_city_info(&client.uid, &client.redis)?));
        out_data.insert("hs".to_owned(), Value::Object(house::get_room(&client.uid, room, &client.redis)?));
        let mut v = Vec::new();
        v.push(Value::String("frn.save".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn type_add(&self, item: &HashMap<String, Value>, room: &str, client: &Client) -> Result<(), Box<dyn Error>> {
        let mut con = client.redis.get_connection()?;
        let tpid = item.get("tpid").ok_or("err")?.get_string()?;
        let oid = item.get("oid").ok_or("err")?.get_i32()?;
        if !ITEMS.contains_key(&tpid) {
            return Ok(())
        }
        if !inventory::take_item(&client.redis, &client.uid, &tpid, 1)? {
            return Ok(())
        }
        let item_object = ITEMS.get(&tpid).unwrap();
        let items: Vec<String> = con.smembers(format!("rooms:{}:{}:items", &client.uid, room)).unwrap_or(Vec::new());
        let mut removed_items = HashSet::new();
        for name in &items {
            let splitted: Vec<&str> = name.split("_").collect();
            let tpid_tmp = splitted[0];
            if !ITEMS.contains_key(tpid_tmp) {
                continue;
            }
            let item_tmp = ITEMS.get(tpid_tmp).unwrap();
            if item_object.category == item_tmp.category {
                if removed_items.contains(tpid_tmp) {
                    continue;
                }
                self.remove_item(&client.uid, &name, room, &client.redis)?;
                inventory::add_item(&client.redis, &client.uid, tpid_tmp, "frn", 1)?;
                removed_items.insert(tpid_tmp.to_owned());
            }
        }
        let mut item = item.clone();
        item.insert("x".to_owned(), Value::F64(0.0));
        item.insert("y".to_owned(), Value::F64(0.0));
        item.insert("z".to_owned(), Value::F64(0.0));
        match item_object.category.as_str() {
            "1" => { // wall
                item.insert("d".to_owned(), Value::I32(3));
                self.add_item(&client.uid, &item, room, &client.redis)?;
                item.insert("x".to_owned(), Value::F64(13.0));
                item.insert("d".to_owned(), Value::I32(5));
                item.insert("oid".to_owned(), Value::I32(oid+1));
                self.add_item(&client.uid, &item, room, &client.redis)?;
            },
            "4" => { // floor
                item.insert("d".to_owned(), Value::I32(5));
                self.add_item(&client.uid, &item, room, &client.redis)?;
            },
            _ => {
                println!("???");
                println!("{}", &item_object.category);
                return Ok(())
            }
        }
        Ok(())
    }

    fn type_update(&self, item: &HashMap<String, Value>, room: &str, client: &Client) -> Result<(), Box<dyn Error>> {
        let mut con = client.redis.get_connection()?;
        let tpid = item.get("tpid").ok_or("err")?.get_string()?;
        let oid = item.get("oid").ok_or("err")?.get_i32()?;
        let items: HashSet<String> = con.smembers(format!("rooms:{}:{}:items", &client.uid, room)).unwrap_or(HashSet::new());
        if items.contains(format!("{}_{}", &tpid, &oid).as_str()) {
            self.update_item(&client.uid, item, room, &client.redis)?;
        }
        else {
            if inventory::take_item(&client.redis, &client.uid, &tpid, 1)? {
                self.add_item(&client.uid, item, room, &client.redis)?;
            }
        }
        Ok(())
    }

    fn type_remove(&self, item: &HashMap<String, Value>, room: &str, client: &Client) -> Result<(), Box<dyn Error>> {
        let mut con = client.redis.get_connection()?;
        let tpid = item.get("tpid").ok_or("err")?.get_string()?;
        let oid = item.get("oid").ok_or("err")?.get_i32()?;
        let items: HashSet<String> = con.smembers(format!("rooms:{}:{}:items", &client.uid, room)).unwrap_or(HashSet::new());
        let name = format!("{}_{}", &tpid, &oid);
        if !items.contains(&name) {
            println!("not contains {}", &name);
            return Ok(())
        }
        self.remove_item(&client.uid, &name, room, &client.redis)?;
        inventory::add_item(&client.redis, &client.uid, &tpid, "frn", 1)?;
        Ok(())
    }

    fn add_item(&self, uid: &str, item: &HashMap<String, Value>,
                room: &str, redis: &redis::Client) -> Result<(), Box<dyn Error>> {
        let mut con = redis.get_connection()?;
        let tpid = item.get("tpid").ok_or("err")?.get_string()?;
        let oid = item.get("oid").ok_or("err")?.get_i32()?;
        let x = item.get("x").ok_or("err")?.get_f64()?;
        let y = item.get("y").ok_or("err")?.get_f64()?;
        let z = item.get("z").ok_or("err")?.get_f64()?;
        let direction = item.get("d").ok_or("err")?.get_i32()?;
        let addr = format!("{}_{}", tpid, oid);
        let _: () = con.sadd(format!("rooms:{}:{}:items", uid, room), &addr)?;
        let _: () = con.rpush(format!("rooms:{}:{}:items:{}", uid, room, &addr),
                              vec![x, y, z])?;
        let _: () = con.rpush(format!("rooms:{}:{}:items:{}", uid, room, &addr),
                              direction)?;
        if item.contains_key("rid") {
            let rid = item.get("rid").unwrap().get_string()?;
            let _: () = con.sadd(format!("rooms:{}:{}:items:{}:options", uid, room, &addr), "rid")?;
            let _: () = con.set(format!("rooms:{}:{}:items:{}:rid", uid, room, &addr), rid)?;
        }
        Ok(())
    }

    fn update_item(&self, uid: &str, item: &HashMap<String, Value>,
                   room: &str, redis: &redis::Client) -> Result<(), Box<dyn Error>> {
        let mut con = redis.get_connection()?;
        let tpid = item.get("tpid").ok_or("err")?.get_string()?;
        let oid = item.get("oid").ok_or("err")?.get_i32()?;
        let x = item.get("x").ok_or("err")?.get_f64()?;
        let y = item.get("y").ok_or("err")?.get_f64()?;
        let z = item.get("z").ok_or("err")?.get_f64()?;
        let direction = item.get("d").ok_or("err")?.get_i32()?;
        let addr = format!("{}_{}", tpid, oid);
        let _: () = con.del(format!("rooms:{}:{}:items:{}", uid, room, addr))?;
        let _: () = con.rpush(format!("rooms:{}:{}:items:{}", uid, room, addr),
                              vec![x, y, z])?;
        let _: () = con.rpush(format!("rooms:{}:{}:items:{}", uid, room, addr),
                              direction)?;
        Ok(())
    }
    
    fn remove_item(&self, uid: &str, addr: &str, room: &str,
                   redis: &redis::Client) -> Result<(), Box<dyn Error>> {
        let mut con = redis.get_connection()?;
        let _: () = con.srem(format!("rooms:{}:{}:items", uid, room), addr)?;
        let _: () = con.del(format!("rooms:{}:{}:items:{}", uid, room, addr))?;
        let options: HashSet<String> = con.smembers(format!("rooms:{}:{}:items:{}:options",
                                                            uid, room, addr))?;
        for option in options {
            let _: () = con.del(format!("rooms:{}:{}:items:{}:{}",
                                        uid, room, addr, option))?;
        }
        let _: () = con.del(format!("rooms:{}:{}:items:{}:options",
                                    uid, room, addr))?;
        Ok(())
    }

    fn buy(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let item = data.get("tpid").ok_or("err")?.get_string()?;
        let count = data.get("cnt").ok_or("err")?.get_i32()?;
        if !ITEMS.contains_key(&item) {
            return Ok(())
        }
        let to_buy = ITEMS.get(&item).unwrap();
        let mut con = client.redis.get_connection()?;
        let gold: i32 = con.get(format!("uid:{}:gld", &client.uid))?;
        let silver: i32 = con.get(format!("uid:{}:slvr", &client.uid))?;
        if to_buy.gold*count > gold || to_buy.silver*count > silver {
            return Ok(())
        }
        let _: () = con.incr(format!("uid:{}:gld", &client.uid), -to_buy.gold*count)?;
        let _: () = con.incr(format!("uid:{}:slvr", &client.uid), -to_buy.silver*count)?;
        inventory::add_item(&client.redis, &client.uid, &to_buy.name, "frn", count)?;
        notify::update_resources(client)?;
        let final_count = inventory::get_item(&client.redis, &client.uid, &item)?.unwrap();
        let mut it = HashMap::new();
        it.insert("c".to_owned(), Value::I32(final_count));
        it.insert("lid".to_owned(), Value::String("".to_owned()));
        it.insert("tid".to_owned(), Value::String(item));
        let mut out_data = HashMap::new();
        out_data.insert("it".to_owned(), Value::Object(it));
        let mut v = Vec::new();
        v.push(Value::String("ntf.inv".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn rename_room(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let id = data.get("id").ok_or("err")?.get_string()?;
        let name = data.get("nm").ok_or("err")?.get_string()?;
        let mut con = client.redis.get_connection()?;
        let rooms: HashSet<String> = con.smembers(format!("rooms:{}", &client.uid))?;
        if !rooms.contains(&id) {
            return Err(Box::from(format!("Room {} not found for {}", &id, &client.uid)))
        }
        let _: () = con.lset(format!("rooms:{}:{}", &client.uid, &id), 0, &name)?;
        let mut out_data = HashMap::new();
        out_data.insert("id".to_owned(), Value::String(id));
        out_data.insert("nm".to_owned(), Value::String(name));
        let mut v = Vec::new();
        v.push(Value::String("frn.rnmrm".to_owned()));
        v.push(Value::Object(out_data));
        client.send(&v, 34)?;
        Ok(())
    }
}

impl Base for Furniture {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "save" => self.save(client, msg)?,
            "buy" => self.buy(client, msg)?,
            "rnmrm" => self.rename_room(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
