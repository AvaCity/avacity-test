use std::collections::HashMap;
use crate::common;

pub fn wrong_pass() -> Vec<common::Value> {
    let mut v = Vec::new();
    v.push(common::Value::I32(5));
    v.push(common::Value::String("Key is invalid".to_owned()));
    v.push(common::Value::Object(HashMap::new()));
    return v
}

pub fn already_joined() -> Vec<common::Value> {
    let mut v = Vec::new();
    v.push(common::Value::I32(6));
    v.push(common::Value::String("Reconnecting".to_owned()));
    v.push(common::Value::Object(HashMap::new()));
    return v
}

pub fn kick_join() -> Vec<common::Value> {
    let mut v = Vec::new();
    v.push(common::Value::I32(6));
    v.push(common::Value::Object(HashMap::new()));
    return v
}
