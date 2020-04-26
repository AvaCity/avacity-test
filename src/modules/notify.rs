use std::collections::HashMap;
use redis::Commands;
use crate::client::Client;
use crate::common::Value;

pub fn get_res(uid: &str, redis: &redis::Client) -> HashMap<String, Value> {
    let mut con = redis.get_connection().unwrap();
    let silver: i32 = con.get(format!("uid:{}:slvr", uid)).unwrap_or(0);
    let gold: i32 = con.get(format!("uid:{}:gld", uid)).unwrap_or(0);
    let energy: i32 = con.get(format!("uid:{}:enrg", uid)).unwrap_or(0);
    let mut res = HashMap::new();
    res.insert("slvr".to_owned(), Value::I32(silver));
    res.insert("gld".to_owned(), Value::I32(gold));
    res.insert("enrg".to_owned(), Value::I32(energy));
    res.insert("emd".to_owned(), Value::I32(0));
    return res;
}

pub fn update_resources(client: &Client) {
    let res = get_res(&client.uid, &client.redis);
    let mut out_data = HashMap::new();
    out_data.insert("res".to_owned(), Value::Object(res));
    let mut v = Vec::new();
    v.push(Value::String("ntf.res".to_owned()));
    v.push(Value::Object(out_data));
    client.send(v, 34);
}
