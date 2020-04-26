extern crate crc;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::convert::TryInto;
use std::error::Error;
use crc::{crc32, Hasher32};
use crate::common;
use crate::errors;

pub fn decode(data: &[u8]) -> Result<HashMap<String, common::Value>, Box<dyn Error>> {
    let mut buffer = Cursor::new(data);
    let mut data = HashMap::new();
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp)?;
    let mask = u8::from_be_bytes(tmp);
    let checksummed = mask & 1 << 3 != 0;
    if checksummed {
        let mut tmp = [0; 4];
        buffer.read_exact(&mut tmp)?;
        let checksum = u32::from_be_bytes(tmp);
        let mut tmp = Vec::new();
        let pos = buffer.position();
        buffer.read_to_end(&mut tmp)?;
        buffer.set_position(pos);
        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest.write(&tmp);
        let real_checksum = digest.sum32();
        if checksum != real_checksum {
            return Err(Box::new(errors::WrongChecksum()));
        }
    }
    buffer.set_position(buffer.position() + 4); // client message number
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp)?;
    let msg_type = common::Value::U8(u8::from_be_bytes(tmp));
    data.insert("type".to_string(), msg_type);
    let msg = common::Value::Vector(decode_vector(&mut buffer)?);
    data.insert("msg".to_string(), msg);
    Ok(data)
}

fn decode_vector(mut buffer: &mut Cursor<&[u8]>) -> Result<Vec<common::Value>, Box<dyn Error>> {
    let mut vector = Vec::new();
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp)?;
    let length = i32::from_be_bytes(tmp);
    for _ in 0..length{
        vector.push(decode_value(&mut buffer)?);
    }
    Ok(vector)
}

fn decode_value(mut buffer: &mut Cursor<&[u8]>) -> Result<common::Value, Box<dyn Error>> {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp)?;
    let datatype = i8::from_be_bytes(tmp);
    match datatype {
        0 => Ok(common::Value::None),
        1 => Ok(common::Value::Boolean(decode_boolean(&mut buffer)?)),
        2 => Ok(common::Value::I32(decode_int32(&mut buffer)?)),
        3 => Ok(common::Value::I64(decode_int64(&mut buffer)?)),
        4 => Ok(common::Value::F64(decode_float64(&mut buffer)?)),
        5 => Ok(common::Value::String(decode_string(&mut buffer)?)),
        6 => Ok(common::Value::Object(decode_object(&mut buffer)?)),
        7 => Ok(common::Value::Vector(decode_vector(&mut buffer)?)),
        _ => Ok(common::Value::None)
    }
}

fn decode_boolean(buffer: &mut Cursor<&[u8]>) -> Result<bool, Box<dyn Error>> {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp)?;
    Ok(u8::from_be_bytes(tmp) != 0)
}

fn decode_int32(buffer: &mut Cursor<&[u8]>) -> Result<i32, Box<dyn Error>> {
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp)?;
    Ok(i32::from_be_bytes(tmp))
}

fn decode_int64(buffer: &mut Cursor<&[u8]>) -> Result<i64, Box<dyn Error>> {
    let mut tmp = [0; 8];
    buffer.read_exact(&mut tmp)?;
    Ok(i64::from_be_bytes(tmp))
}

fn decode_float64(buffer: &mut Cursor<&[u8]>) -> Result<f64, Box<dyn Error>> {
    let mut tmp = [0; 8];
    buffer.read_exact(&mut tmp)?;
    Ok(f64::from_be_bytes(tmp))
}

fn decode_object(mut buffer: &mut Cursor<&[u8]>) -> Result<HashMap<String, common::Value>, Box<dyn Error>> {
    let mut tmp = [0; 4];
    buffer.read_exact(&mut tmp)?;
    let fields = i32::from_be_bytes(tmp);
    let mut object = HashMap::new();
    for _ in 0..fields {
        let key = decode_string(&mut buffer)?;
        object.insert(key, decode_value(&mut buffer)?);
    }
    Ok(object)
}

fn decode_string(buffer: &mut Cursor<&[u8]>) -> Result<String, Box<dyn Error>> {
    let mut tmp = [0; 1];
    buffer.read_exact(&mut tmp)?;
    let mut i: u32 = 0;
    let mut b = u8::from_be_bytes(tmp);
    let mut value: i32 = 0;
    while b & 128 != 0 {
        value += ((b as i32) & 127) << i;
        i += 7;
        let mut tmp = [0; 1];
        buffer.read_exact(&mut tmp)?;
        b = u8::from_be_bytes(tmp);
    }
    let length: i32 = value | (b as i32) << i;
    let mut tmp = vec![0; length.try_into()?];
    buffer.read_exact(&mut tmp)?;
    Ok(String::from_utf8(tmp)?.to_owned())
}
