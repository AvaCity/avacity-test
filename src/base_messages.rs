use std::collections::HashMap;
use crate::common;

pub fn wrong_pass() -> Vec<common::Value> {
    let mut vec = Vec::new();
    vec.push(common::Value::I32(5));
    vec.push(common::Value::String("Key is invalid".to_owned()));
    vec.push(common::Value::Object(HashMap::new()));
    return vec;
}
