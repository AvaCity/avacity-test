use std::collections::{HashMap, HashSet};
use std::error::Error;
use redis::Commands;
use crate::common::Value;
use crate::client::Client;
use crate::inventory;
use crate::modules::{Base, get_appearance};

const COLLECTIONS: &'static [&'static str] = &["casual", "club", "official", "swimwear", "underdress"];

pub struct Avatar {
    pub prefix: &'static str
}

impl Avatar {
    pub fn new() -> Avatar {
        Avatar {
            prefix: "a"
        }
    }
 
    fn appearance(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "save" => self.appearance_save(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }

    fn appearance_save(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let apprnc = data.get("apprnc").ok_or("key not found")?.get_object()?;
        let old_apprnc = get_appearance(&client.uid, &client.redis)?;
        match old_apprnc {
            Some(_) => self.update_appearance(client, &apprnc)?,
            None => self.create_avatar(client, &apprnc)?
        }
        let mut v = Vec::new();
        v.push(Value::String("a.apprnc.save".to_owned()));
        let mut data = HashMap::new();
        data.insert("apprnc".to_owned(), Value::Object(get_appearance(&client.uid, &client.redis)?.unwrap()));
        v.push(Value::Object(data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn update_appearance(&self, client: &Client, apprnc: &HashMap<String, Value>) -> Result<(), Box<dyn Error>>{
        let mut con = client.redis.get_connection()?;
        let key = format!("uid:{}:appearance", &client.uid);
        let _: () = con.del(&key)?;
        let _: () = con.rpush(&key, apprnc.get("n").ok_or("key not found")?.get_string()?)?;
        let _: () = con.rpush(&key, apprnc.get("nct").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("g").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("sc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("ht").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("hc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("brt").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("brc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("et").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("ec").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("fft").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("fat").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("fac").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("ss").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("ssc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("mt").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("mc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("sh").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("shc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("rg").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("rc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("pt").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("pc").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("bt").ok_or("key not found")?.get_i32()?)?;
        let _: () = con.rpush(&key, apprnc.get("bc").ok_or("key not found")?.get_i32()?)?;
        Ok(())
    }

    fn create_avatar(&self, client: &Client, apprnc: &HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
        self.update_appearance(client, apprnc)?;
        let gender = apprnc.get("g").ok_or("key not found")?.get_i32()?;
        let mut con = client.redis.get_connection()?;
        let _: () = con.set(format!("uid:{}:wearing", &client.uid), "casual")?;
        match gender {
            1 => { // Boy
                inventory::add_item(&client.redis, &client.uid, "boyUnderdress1", "cls", 1)?;
                for cloth in &["boyShoes8", "boyPants10", "boyShirt14"] {
                    inventory::add_item(&client.redis, &client.uid, cloth, "cls", 1)?;
                    inventory::set_wearing(&client.redis, &client.uid, cloth, true)?;
                }
            },
            2 => { // Girl
                inventory::add_item(&client.redis, &client.uid, "girlUnderdress1", "cls", 1)?;
                inventory::add_item(&client.redis, &client.uid, "girlUnderdress2", "cls", 1)?;
                for cloth in &["girlShoes14", "girlPants9", "girlShirt12"] {
                    inventory::add_item(&client.redis, &client.uid, cloth, "cls", 1)?;
                    inventory::set_wearing(&client.redis, &client.uid, cloth, true)?;
                }
            },
            _ => panic!("Wrong gender")
        }
        Ok(())
    }

    fn clothes(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "wear" => self.wear_clothes(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }

    fn wear_clothes(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let data = msg[2].get_object()?;
        let clothes = data.get("clths").ok_or("err")?.get_vector()?;
        let collection = data.get("ctp").ok_or("err")?.get_string()?;
        if COLLECTIONS.iter().find(|x| *x == &collection).is_none() {
            println!("Collection {} does not exists", collection);
            return Ok(())
        }
        let mut con = client.redis.get_connection()?;
        let items: HashSet<String> = con.smembers(format!("uid:{}:items", &client.uid))?;
        let mut to_wear = Vec::new();
        for tmp in clothes {
            let cloth = tmp.get_object()?;
            let tpid = cloth.get("tpid").ok_or("err")?.get_string()?;
            let clid = cloth.get("clid").ok_or("err")?;
            let mut name = tpid.clone();
            if let Value::String(v) = clid {
                let clid = v.clone();
                if !clid.is_empty() {
                    name = format!("{}_{}", tpid, clid);
                }
            }
            if !items.contains(&name) {
                println!("Item {} not found for uid {}", &name, &client.uid);
                return Ok(())
            }
            to_wear.push(name)
        }
        let _: () = con.set(format!("uid:{}:wearing", &client.uid), &collection)?;
        let weared_clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", &client.uid, &collection))?;
        for cloth in weared_clothes {
            inventory::set_wearing(&client.redis, &client.uid, &cloth, false)?;
        }
        for cloth in to_wear {
            inventory::set_wearing(&client.redis, &client.uid, &cloth, true)?;
        }
        let mut data = HashMap::new();
        data.insert("inv".to_owned(), Value::Object(inventory::get(&client.uid, &client.redis)?));
        data.insert("clths".to_owned(), Value::Object(inventory::get_clths(&client.uid, &client.redis)?));
        data.insert("ccltn".to_owned(), Value::Object(inventory::get_collection(&client.uid, &client.redis)?));
        data.insert("ctp".to_owned(), Value::String(collection.clone()));
        data.insert("cn".to_owned(), Value::String("".to_owned()));
        let mut v = Vec::new();
        v.push(Value::String("a.clths.wear".to_owned()));
        v.push(Value::Object(data));
        client.send(&v, 34)?;
        Ok(())
    }
}

impl Base for Avatar {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "apprnc" => self.appearance(client, msg)?,
            "clths" => self.clothes(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
