use std::collections::HashMap;
use redis::Commands;
use crate::client::Client;
use crate::common::Value;
pub mod house;

pub trait Base: Send{
    fn handle(&self, client: &Client, msg: &Vec<Value>);
}

pub fn get_appearance(uid: &String, redis: &redis::Client) -> Option<HashMap<String, Value>> {
    let mut con = redis.get_connection().unwrap();
    let apprnc: Option<Vec<String>> = con.lrange(format!("uid:{}:appearance", uid),
                                                 0, -1).unwrap();
    match apprnc {
        Some(vec) => {
            let mut out: HashMap<String, Value> = HashMap::new();
            if vec.len() == 0 {
                return None;
            }
            out.insert("n".to_owned(), Value::String(vec[0].clone()));
            out.insert("nct".to_owned(), Value::I32(vec[1].parse::<i32>().unwrap()));
            out.insert("g".to_owned(), Value::I32(vec[2].parse::<i32>().unwrap()));
            out.insert("sc".to_owned(), Value::I32(vec[3].parse::<i32>().unwrap()));
            out.insert("ht".to_owned(), Value::I32(vec[4].parse::<i32>().unwrap()));
            out.insert("hc".to_owned(), Value::I32(vec[5].parse::<i32>().unwrap()));
            out.insert("brt".to_owned(), Value::I32(vec[6].parse::<i32>().unwrap()));
            out.insert("brc".to_owned(), Value::I32(vec[7].parse::<i32>().unwrap()));
            out.insert("et".to_owned(), Value::I32(vec[8].parse::<i32>().unwrap()));
            out.insert("ec".to_owned(), Value::I32(vec[9].parse::<i32>().unwrap()));
            out.insert("fft".to_owned(), Value::I32(vec[10].parse::<i32>().unwrap()));
            out.insert("fat".to_owned(), Value::I32(vec[11].parse::<i32>().unwrap()));
            out.insert("fac".to_owned(), Value::I32(vec[12].parse::<i32>().unwrap()));
            out.insert("ss".to_owned(), Value::I32(vec[13].parse::<i32>().unwrap()));
            out.insert("ssc".to_owned(), Value::I32(vec[14].parse::<i32>().unwrap()));
            out.insert("mt".to_owned(), Value::I32(vec[15].parse::<i32>().unwrap()));
            out.insert("mc".to_owned(), Value::I32(vec[16].parse::<i32>().unwrap()));
            out.insert("sh".to_owned(), Value::I32(vec[17].parse::<i32>().unwrap()));
            out.insert("shc".to_owned(), Value::I32(vec[18].parse::<i32>().unwrap()));
            out.insert("rg".to_owned(), Value::I32(vec[19].parse::<i32>().unwrap()));
            out.insert("rc".to_owned(), Value::I32(vec[20].parse::<i32>().unwrap()));
            out.insert("pt".to_owned(), Value::I32(vec[21].parse::<i32>().unwrap()));
            out.insert("pc".to_owned(), Value::I32(vec[22].parse::<i32>().unwrap()));
            out.insert("bt".to_owned(), Value::I32(vec[23].parse::<i32>().unwrap()));
            out.insert("bc".to_owned(), Value::I32(vec[24].parse::<i32>().unwrap()));
            Some(out)
        }
        None => {
            None
        }
    }
}
