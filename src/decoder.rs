extern crate crc;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::str;
use std::convert::TryInto;
use crc::{crc32, Hasher32};
use crate::common;

pub fn decode(data: &[u8]) -> Result<HashMap<String, common::Value>, &str> {
    let mut buffer = Cursor::new(data);
    let mut data = HashMap::new();
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp).unwrap();
    let mask = u8::from_be_bytes(tmp);
    let checksummed = mask & 1 << 3 != 0;
    if checksummed {
        let mut tmp = [0; 4];
        buffer.read_exact(&mut tmp).unwrap();
        let checksum = u32::from_be_bytes(tmp);
        let mut tmp = Vec::new();
        let pos = buffer.position();
        buffer.read_to_end(&mut tmp).unwrap();
        buffer.set_position(pos);
        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest.write(&tmp);
        let real_checksum = digest.sum32();
        if checksum != real_checksum {
            return Err("Wrong checksum");
        }
    }
    buffer.set_position(buffer.position() + 4); // client message number
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp).unwrap();
    let msg_type = common::Value::U8(u8::from_be_bytes(tmp));
    data.insert("type".to_string(), msg_type);
    let msg = common::Value::Vector(decode_vector(&mut buffer));
    data.insert("msg".to_string(), msg);
    Ok(data)
}

fn decode_vector(mut buffer: &mut Cursor<&[u8]>) -> Vec<common::Value> {
    let mut vector = Vec::new();
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp).unwrap();
    let length = i32::from_be_bytes(tmp);
    for _ in 0..length{
        vector.push(decode_value(&mut buffer));
    }
    return vector;
}

fn decode_value(mut buffer: &mut Cursor<&[u8]>) -> common::Value {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp).unwrap();
    let datatype = i8::from_be_bytes(tmp);
    match datatype {
        0 => return common::Value::None,
        1 => return common::Value::Boolean(decode_boolean(&mut buffer)),
        2 => return common::Value::I32(decode_int32(&mut buffer)),
        3 => return common::Value::I64(decode_int64(&mut buffer)),
        4 => return common::Value::F32(decode_float32(&mut buffer)),
        5 => return common::Value::String(decode_string(&mut buffer)),
        6 => return common::Value::Object(decode_object(&mut buffer)),
        7 => return common::Value::Vector(decode_vector(&mut buffer)),
        _ => return common::Value::None
    }
}

fn decode_boolean(buffer: &mut Cursor<&[u8]>) -> bool {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp).unwrap();
    return u8::from_be_bytes(tmp) != 0;
}

fn decode_int32(buffer: &mut Cursor<&[u8]>) -> i32 {
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp).unwrap();
    return i32::from_be_bytes(tmp);
}

fn decode_int64(buffer: &mut Cursor<&[u8]>) -> i64 {
    let mut tmp = [0; 8];
    buffer.read_exact(&mut tmp).unwrap();
    return i64::from_be_bytes(tmp);
}

fn decode_float32(buffer: &mut Cursor<&[u8]>) -> f32 {
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp).unwrap();
    return f32::from_be_bytes(tmp);
}

fn decode_object(mut buffer: &mut Cursor<&[u8]>) -> HashMap<String, common::Value> {
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp).unwrap();
    let fields = i32::from_be_bytes(tmp);
    let mut object = HashMap::new();
    for _ in 0..fields {
        let key = decode_string(&mut buffer);
        object.insert(key, decode_value(&mut buffer));
    }
    return object;
}

fn decode_string(buffer: &mut Cursor<&[u8]>) -> String {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp).unwrap();
    let mut i: u32 = 0;
    let mut b = u8::from_be_bytes(tmp);
    let mut value: i32 = 0;
    while b & 128 != 0 {
        value += ((b as i32) & 127) << i;
        i += 7;
        let mut tmp = [0; 1];
        buffer.read_exact(&mut tmp).unwrap();
        b = u8::from_be_bytes(tmp);
    }
    let length: i32 = value | (b as i32) << i;
    let mut tmp = vec![0; length.try_into().unwrap()];
    buffer.read_exact(&mut tmp).unwrap();
    return String::from_utf8(tmp).unwrap().to_owned();
}
