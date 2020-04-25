use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    None,
    Boolean(bool),
    String(String),
    U8(u8),
    I32(i32),
    I64(i64),
    F32(f32),
    Vector(Vec<Value>),
    Object(HashMap<String, Value>)
}

impl Value {
    pub fn get_vector(&self) -> Result<&Vec<Value>, &'static str> {
        if let Value::Vector(v) = self {
            return Ok(v);
        } else {
            return Err("Can't convert");
        }
    }
    pub fn get_u8(&self) -> Result<u8, &'static str> {
        if let Value::U8(v) = self {
            return Ok(*v);
        } else {
            return Err("Can't convert");
        }
    }
    pub fn get_i32(&self) -> Result<i32, &'static str> {
        if let Value::I32(v) = self {
            return Ok(*v);
        } else {
            return Err("Can't convert");
        }
    }
    pub fn get_string(&self) -> Result<String, &'static str> {
        if let Value::String(v) = self {
            return Ok(v.clone());
        } else {
            return Err("Can't convert");
        }
    }
    pub fn get_object(&self) -> Result<&HashMap<String, Value>, &'static str> {
        if let Value::Object(v) = self {
            return Ok(v);
        } else {
            return Err("Can't convert");
        }
    }
}
