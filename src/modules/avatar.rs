use std::collections::HashMap;
use redis::Commands;
use crate::common::Value;
use crate::client::Client;
use crate::inventory;
use crate::modules::{Base, get_appearance};

pub struct Avatar {
    pub prefix: &'static str
}

impl Avatar {
    pub fn new() -> Avatar {
        Avatar {
            prefix: "a"
        }
    }
 
    fn appearance(&self, client: &Client, msg: &Vec<Value>) {
        let tmp = msg[1].get_string().unwrap();
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[2];
        match command {
            "save" => self.appearance_save(client, msg),
            _ => println!("Command {} not found", tmp)
        }
    }

    fn appearance_save(&self, client: &Client, msg: &Vec<Value>) {
        let data = msg[2].get_object().unwrap();
        let apprnc = data.get("apprnc").unwrap().get_object().unwrap();
        let old_apprnc = get_appearance(&client.uid, &client.redis);
        match old_apprnc {
            Some(_) => self.update_appearance(client, &apprnc),
            None => self.create_avatar(client, &apprnc)
        }
        let mut v = Vec::new();
        v.push(Value::String("a.apprnc.save".to_owned()));
        let mut data = HashMap::new();
        data.insert("apprnc".to_owned(), Value::Object(get_appearance(&client.uid, &client.redis).unwrap()));
        v.push(Value::Object(data));
        client.send(v, 34)
    }

    fn update_appearance(&self, client: &Client, apprnc: &HashMap<String, Value>) {
        let mut con = client.redis.get_connection().unwrap();
        let key = format!("uid:{}:appearance", &client.uid);
        let _: () = con.del(&key).unwrap();
        let _: () = con.rpush(&key, apprnc.get("n").unwrap().get_string().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("nct").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("g").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("sc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("ht").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("hc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("brt").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("brc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("et").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("ec").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("fft").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("fat").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("fac").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("ss").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("ssc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("mt").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("mc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("sh").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("shc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("rg").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("rc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("pt").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("pc").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("bt").unwrap().get_i32().unwrap()).unwrap();
        let _: () = con.rpush(&key, apprnc.get("bc").unwrap().get_i32().unwrap()).unwrap();
    }

    fn create_avatar(&self, client: &Client, apprnc: &HashMap<String, Value>) {
        self.update_appearance(client, apprnc);
        let gender = apprnc.get("g").unwrap().get_i32().unwrap();
        let mut con = client.redis.get_connection().unwrap();
        let _: () = con.set(format!("uid:{}:wearing", &client.uid), "casual").unwrap();
        match gender {
            1 => { // Boy
                inventory::add_item(&client.redis, &client.uid, "boyUnderdress1", "cls", 1);
                for cloth in &["boyShoes8", "boyPants10", "boyShirt14"] {
                    inventory::add_item(&client.redis, &client.uid, cloth, "cls", 1);
                    inventory::set_wearing(&client.redis, &client.uid, cloth, true);
                }
            },
            2 => { // Girl
                inventory::add_item(&client.redis, &client.uid, "girlUnderdress1", "cls", 1);
                inventory::add_item(&client.redis, &client.uid, "girlUnderdress2", "cls", 1);
                for cloth in &["girlShoes14", "girlPants9", "girlShirt12"] {
                    inventory::add_item(&client.redis, &client.uid, cloth, "cls", 1);
                    inventory::set_wearing(&client.redis, &client.uid, cloth, true);
                }
            },
            _ => panic!("Wrong gender")
        }
    }
}

impl Base for Avatar {
    fn handle(&self, client: &Client, msg: &Vec<Value>) {
        let tmp = msg[1].get_string().unwrap();
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "apprnc" => self.appearance(client, msg),
            _ => println!("Command {} not found", tmp)
        }
    }
}
