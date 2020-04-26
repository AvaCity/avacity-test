use std::collections::{HashMap, HashSet};
use std::error::Error;
use redis::Commands;
use crate::common::Value;

pub fn set_wearing(redis: &redis::Client, uid: &str, item: &str, wearing: bool) -> Result<(), Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    if get_item(redis, uid, item)?.is_none() {
        return Ok(());
    }
    let collection: String = con.get(format!("uid:{}:wearing", uid))?;
    if wearing {
        let _: () = con.sadd(format!("uid:{}:{}", uid, collection), item)?;
    }
    else {
        let _: () = con.srem(format!("uid:{}:{}", uid, collection), item)?;
    }
    Ok(())
}

pub fn add_item(redis: &redis::Client, uid: &str, item: &str, type_: &str, count: i32) -> Result<(), Box<dyn Error>> {
    if type_ == "cls".to_owned() && get_item(redis, uid, item)?.is_some() {
        return Ok(());
    }
    let mut con = redis.get_connection()?;
    let _: () = con.rpush(format!("uid:{}:items:{}", uid, item), type_)?;
    let _: () = con.rpush(format!("uid:{}:items:{}", uid, item), count)?;
    Ok(())
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

pub fn get(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let mut frn_it = Vec::new();
    let mut act_it = Vec::new();
    let mut gm_it = Vec::new();
    let mut lt_it = Vec::new();
    let mut cls_it = Vec::new();
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
            "cls" => cls_it.push(Value::Object(out_item)),
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
