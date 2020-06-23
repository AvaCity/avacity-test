use std::collections::{HashMap, HashSet};
use std::error::Error;
use redis::Commands;
use crate::common::Value;
use crate::modules::{get_gender, avatar};
use crate::parser;

lazy_static! {
    static ref CONFLICTS: Vec<[String; 2]> = parser::parse_conflicts();
}

pub fn set_wearing(redis: &redis::Client, uid: &str, item: &str, wearing: bool) -> Result<(), Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    if get_item(redis, uid, item)?.is_none() {
        return Ok(());
    }
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    if wearing {
        check_conflicts(redis, uid, item)?;
        let _: () = con.sadd(format!("uid:{}:{}", uid, collection), item)?;
    }
    else {
        let _: () = con.srem(format!("uid:{}:{}", uid, collection), item)?;
    }
    Ok(())
}

fn check_conflicts(redis: &redis::Client, uid: &str, item: &str) -> Result<(), Box<dyn Error>> {
    let cloth_list = match get_gender(uid, redis)? {
        "boy" => avatar::CLOTHES.get("boy").unwrap(),
        "girl" => avatar::CLOTHES.get("girl").unwrap(),
        _ => return Err(Box::from("Gender not found"))
    };
    let mut con = redis.get_connection()?;
    let tmp: Vec<&str> = item.split("_").collect();
    let cloth = cloth_list.get(tmp[0]).ok_or("err")?;
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    let clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", uid, collection))?;
    for weared in clothes {
        let tmp2: Vec<&str> = weared.split("_").collect();
        let weared_cloth = cloth_list.get(tmp2[0]).ok_or("err")?;
        if has_confict(cloth, weared_cloth) {
            let _: () = con.srem(format!("uid:{}:{}", uid, collection), weared)?;
        }
    }
    Ok(())
}

fn has_confict(cloth1: &parser::Item, cloth2: &parser::Item) -> bool {
    if cloth1.category == cloth2.category {
        return true
    }
    for conflict in CONFLICTS.iter() {
        if conflict[0] == cloth1.category && conflict[1] == cloth2.category {
            return true
        }
        else if conflict[1] == cloth1.category && conflict[0] == cloth2.category {
            return true
        }
    }
    return false
}

pub fn add_item(redis: &redis::Client, uid: &str, item: &str, type_: &str, count: i32) -> Result<(), Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let have_count: Option<i32> = get_item(redis, uid, item)?;
    match have_count {
        Some(v) => {
            if type_ == "cls".to_owned() {
                return Ok(());
            }
            let _: () = con.lset(format!("uid:{}:items:{}", uid, item), 1, v+count)?;
        }
        None => {
            let _: () = con.sadd(format!("uid:{}:items", uid), item)?;
            let _: () = con.rpush(format!("uid:{}:items:{}", uid, item), type_)?;
            let _: () = con.rpush(format!("uid:{}:items:{}", uid, item), count)?;
        }
    }
    Ok(())
}

pub fn take_item(redis: &redis::Client, uid: &str, item: &str, count: i32) -> Result<bool, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let have_count: Option<i32> = get_item(redis, uid, item)?;
    match have_count {
        Some(v) => {
            if v < count {
                return Ok(false)
            }
            if v-count == 0 {
                let _: () = con.del(format!("uid:{}:items:{}", uid, item))?;
                let _: () = con.srem(format!("uid:{}:items", uid), item)?;
            }
            else {
                let _: () = con.lset(format!("uid:{}:items:{}", uid, item), 1, v-count)?;
            }
            return Ok(true)
        },
        None => {
            return Ok(false)
        }
    }
}

pub fn get_item(redis: &redis::Client, uid: &str, item: &str) -> Result<Option<i32>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let count: Option<i32> = con.lindex(format!("uid:{}:items:{}", uid, item), 1)?;
    Ok(count)
}

pub fn get_clths(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    let clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", uid, collection))?;
    let mut out_vec = Vec::new();
    for cloth in clothes {
        let mut item = HashMap::new();
        let splitted: Vec<&str> = cloth.split("_").collect();
        item.insert("tpid".to_owned(), Value::String(splitted[0].to_owned()));
        if splitted.len() == 2 {
            item.insert("clid".to_owned(), Value::String(splitted[1].to_owned()));
        }
        else {
            item.insert("clid".to_owned(), Value::None);
        }
        out_vec.push(Value::Object(item));
    }
    let mut out = HashMap::new();
    out.insert("clths".to_owned(), Value::Vector(out_vec));
    Ok(out)
}

pub fn get_all_collections(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let mut collections = HashMap::new();
    for collection in &["casual", "club", "official", "swimwear", "underdress"] {
        let clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", uid, collection))?;
        let mut cct = Vec::new();
        for cloth in clothes {
            let splitted: Vec<&str> = cloth.split("_").collect();
            if splitted.len() == 2 {
                cct.push(Value::String(format!("{}_{}", splitted[0], splitted[1])));
            }
            else {
                cct.push(Value::String(splitted[0].to_string()));
            }
        }
        let mut out = HashMap::new();
        out.insert("cct".to_owned(), Value::Vector(cct));
        out.insert("ctp".to_owned(), Value::String(collection.to_string()));
        out.insert("cn".to_owned(), Value::String("".to_owned()));
        collections.insert(collection.to_string(), Value::Object(out));
    }
    let mut data = HashMap::new();
    let current_collection: String = con.get(format!("uid:{}:wearing", uid))?;
    data.insert("cc".to_owned(), Value::String(current_collection));
    data.insert("ccltns".to_owned(), Value::Object(collections));
    Ok(data)
}

pub fn get_collection(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    let clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", uid, &collection))?;
    let mut cct = Vec::new();
    for cloth in clothes {
        let splitted: Vec<&str> = cloth.split("_").collect();
        if splitted.len() == 2 {
            cct.push(Value::String(format!("{}_{}", splitted[0], splitted[1])));
        }
        else {
            cct.push(Value::String(splitted[0].to_owned()));
        }
    }
    let mut out = HashMap::new();
    out.insert("cct".to_owned(), Value::Vector(cct));
    out.insert("ctp".to_owned(), Value::String(collection));
    out.insert("cn".to_owned(), Value::String("".to_owned()));
    Ok(out)
}

pub fn get(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let mut frn_it = Vec::new();
    let mut act_it = Vec::new();
    let mut gm_it = Vec::new();
    let mut lt_it = Vec::new();
    let mut cls_it = Vec::new();
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    let clothes: HashSet<String> = con.smembers(format!("uid:{}:{}", uid, collection))?;
    let items: HashSet<String> = con.smembers(format!("uid:{}:items", uid))?;
    for name in items {
        let item: Vec<String> = con.lrange(format!("uid:{}:items:{}", uid, &name), 0, -1)?;
        let mut out_item = HashMap::new();
        out_item.insert("c".to_owned(), Value::I32(item[1].parse::<i32>()?));
        let splitted: Vec<&str> = name.split("_").collect();
        out_item.insert("tid".to_owned(), Value::String(splitted[0].to_owned()));
        if splitted.len() == 2 {
            out_item.insert("iid".to_owned(), Value::String(splitted[1].to_owned()));
        }
        else {
            out_item.insert("iid".to_owned(), Value::String("".to_owned()));
        }
        match item[0].as_str() {
            "frn" => frn_it.push(Value::Object(out_item)),
            "act" => act_it.push(Value::Object(out_item)),
            "gm" => gm_it.push(Value::Object(out_item)),
            "lt" => lt_it.push(Value::Object(out_item)),
            "cls" => {
                if clothes.contains(&name) {
                    continue;
                }
                cls_it.push(Value::Object(out_item))
            },
            _ => panic!("Wrong type")
        }
    }
    let mut frn = HashMap::new();
    frn.insert("id".to_owned(), Value::String("frn".to_owned()));
    frn.insert("it".to_owned(), Value::Vector(frn_it));
    let mut act = HashMap::new();
    act.insert("id".to_owned(), Value::String("act".to_owned()));
    act.insert("it".to_owned(), Value::Vector(act_it));
    let mut gm = HashMap::new();
    gm.insert("id".to_owned(), Value::String("gm".to_owned()));
    gm.insert("it".to_owned(), Value::Vector(gm_it));
    let mut lt = HashMap::new();
    lt.insert("id".to_owned(), Value::String("lt".to_owned()));
    lt.insert("it".to_owned(), Value::Vector(lt_it));
    let mut cls = HashMap::new();
    cls.insert("id".to_owned(), Value::String("cls".to_owned()));
    cls.insert("it".to_owned(), Value::Vector(cls_it));
    let mut c = HashMap::new();
    c.insert("frn".to_owned(), Value::Object(frn));
    c.insert("act".to_owned(), Value::Object(act));
    c.insert("gm".to_owned(), Value::Object(gm));
    c.insert("lt".to_owned(), Value::Object(lt));
    c.insert("cls".to_owned(), Value::Object(cls));
    let mut out = HashMap::new();
    out.insert("c".to_owned(), Value::Object(c));
    Ok(out)
}
