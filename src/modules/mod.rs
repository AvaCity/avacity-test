use std::collections::HashMap;
use redis::Commands;
use crate::client::Client;
use crate::common::Value;
pub mod house;

pub trait Base: Send {
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
            out.insert("n".to_owned(), Value::String(vec[0].clone()));                  // name
            out.insert("nct".to_owned(), Value::I32(vec[1].parse::<i32>().unwrap()));   // name change time
            out.insert("g".to_owned(), Value::I32(vec[2].parse::<i32>().unwrap()));     // gender
            out.insert("sc".to_owned(), Value::I32(vec[3].parse::<i32>().unwrap()));    // skin color
            out.insert("ht".to_owned(), Value::I32(vec[4].parse::<i32>().unwrap()));    // hair type
            out.insert("hc".to_owned(), Value::I32(vec[5].parse::<i32>().unwrap()));    // hair color
            out.insert("brt".to_owned(), Value::I32(vec[6].parse::<i32>().unwrap()));   // brows type
            out.insert("brc".to_owned(), Value::I32(vec[7].parse::<i32>().unwrap()));   // brows color
            out.insert("et".to_owned(), Value::I32(vec[8].parse::<i32>().unwrap()));    // eyes type
            out.insert("ec".to_owned(), Value::I32(vec[9].parse::<i32>().unwrap()));    // eyes color
            out.insert("fft".to_owned(), Value::I32(vec[10].parse::<i32>().unwrap()));  // face feature type
            out.insert("fat".to_owned(), Value::I32(vec[11].parse::<i32>().unwrap()));  // face art type
            out.insert("fac".to_owned(), Value::I32(vec[12].parse::<i32>().unwrap()));  // face art color
            out.insert("ss".to_owned(), Value::I32(vec[13].parse::<i32>().unwrap()));   // strass type
            out.insert("ssc".to_owned(), Value::I32(vec[14].parse::<i32>().unwrap()));  // strass color
            out.insert("mt".to_owned(), Value::I32(vec[15].parse::<i32>().unwrap()));   // mouth type
            out.insert("mc".to_owned(), Value::I32(vec[16].parse::<i32>().unwrap()));   // mouth color
            out.insert("sh".to_owned(), Value::I32(vec[17].parse::<i32>().unwrap()));   // shadow type
            out.insert("shc".to_owned(), Value::I32(vec[18].parse::<i32>().unwrap()));  // shadow color
            out.insert("rg".to_owned(), Value::I32(vec[19].parse::<i32>().unwrap()));   // rouge type
            out.insert("rc".to_owned(), Value::I32(vec[20].parse::<i32>().unwrap()));   // rouge color
            out.insert("pt".to_owned(), Value::I32(vec[21].parse::<i32>().unwrap()));   // pirsing type
            out.insert("pc".to_owned(), Value::I32(vec[22].parse::<i32>().unwrap()));   // pirsing color
            out.insert("bt".to_owned(), Value::I32(vec[23].parse::<i32>().unwrap()));   // beard type
            out.insert("bc".to_owned(), Value::I32(vec[24].parse::<i32>().unwrap()));   // beard color
            Some(out)
        }
        None => None
    }
}
