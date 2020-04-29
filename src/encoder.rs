use std::collections::HashMap;
use crate::common;
use bytes::{BytesMut, BufMut};

pub fn encode(data: &Vec<common::Value>, type_: u8) -> Result<Vec<u8>, &'static str> {
    let mut out = BytesMut::new();
    out.put_u8(type_);
    out.put_i32(data.len() as i32);
    for item in data {
        let tmp = encode_value(&item).unwrap();
        out.put(&tmp[..]);
    }
    return Ok(out[..].to_vec());
}

pub fn encode_value(item: &common::Value) -> Result<Vec<u8>, &'static str> {
    match item {
        common::Value::None => return Ok(encode_none()),
        common::Value::Boolean(v) => return Ok(encode_boolean(*v)),
        common::Value::I32(v) => return Ok(encode_int32(*v)),
        common::Value::I64(v) => return Ok(encode_int64(*v)),
        common::Value::F64(v) => return Ok(encode_float(*v)),
        common::Value::Vector(v) => return Ok(encode_vector(v)),
        common::Value::String(v) => return Ok(encode_string(v, false)),
        common::Value::Object(v) => return Ok(encode_object(v)),
        _ => return Err("Type not found"),
    };
}

pub fn encode_none() -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(0);
    return out[..].to_vec();
}

pub fn encode_boolean(item: bool) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(1);
    out.put_u8(item as u8);
    return out[..].to_vec();
}

pub fn encode_int32(item: i32) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(2);
    out.put_i32(item);
    return out[..].to_vec();
}

pub fn encode_int64(item: i64) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(3);
    out.put_i64(item);
    return out[..].to_vec();
}

pub fn encode_float(item: f64) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(4);
    out.put_f64(item);
    return out[..].to_vec();
}

pub fn encode_string(item: &String, for_obj: bool) -> Vec<u8> {
    let mut out = BytesMut::new();
    if !for_obj {
        out.put_u8(5);
    }
    let bytes = item.as_bytes();
    let mut length = bytes.len() as i64;
    while length & 4294967168 != 0 {
        out.put_u8((length & 127 | 128) as u8);
        length = length >> 7;
    }
    out.put_u8((length & 127) as u8);
    out.put(&bytes[..]);
    return out[..].to_vec();
}

pub fn encode_object(data: &HashMap<String, common::Value>) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(6);
    out.put_i32(data.len() as i32);
    for key in data.keys() {
        out.put(&encode_string(key, true)[..]);
        let value = data.get(key).unwrap().clone();
        out.put(&encode_value(&value).unwrap()[..]);
    }
    return out[..].to_vec();
}

pub fn encode_vector(data: &Vec<common::Value>) -> Vec<u8> {
    let mut out = BytesMut::new();
    out.put_u8(7);
    out.put_i32(data.len() as i32);
    for item in data {
        out.put(&encode_value(&item).unwrap()[..]);
    }
    return out[..].to_vec();
}
