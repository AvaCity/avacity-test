use std::collections::HashMap;
use std::error::Error;
use crate::client::Client;
use crate::common::Value;

const PROFESSIONS: bool = true;
const CHAT_DECOR: bool = true;

pub fn new(client: &Client) -> Result<(), Box<dyn Error>> {
    let mut campaigns = Vec::new();
    if PROFESSIONS {
        campaigns.push(professions_campaign());
    }
    if CHAT_DECOR {
        campaigns.push(chat_decor_campaign());
        campaigns.push(chat_decor_shop_campaign());
    }
    let mut data = HashMap::new();
    data.insert("campaigns".to_owned(), Value::Vector(campaigns));
    let mut v = Vec::new();
    v.push(Value::String("cm.new".to_owned()));
    v.push(Value::Object(data));
    client.send(&v, 34)?;
    Ok(())
}

fn professions_campaign() -> Value {
    let mut out = HashMap::new();
    out.insert("st".to_owned(), Value::I32(1));
    out.insert("v".to_owned(), Value::I32(1));
    out.insert("id".to_owned(), Value::I32(114));
    out.insert("iu".to_owned(), Value::String("".to_owned()));
    out.insert("tp".to_owned(), Value::I32(9));
    out.insert("ed".to_owned(), Value::Date(1440622800000));
    let mut cil = Vec::new();
    let mut item = HashMap::new();
    item.insert("sc".to_owned(), Value::I32(0));
    item.insert("gl".to_owned(), Value::I32(0));
    item.insert("si".to_owned(), Value::I32(0));
    item.insert("cid".to_owned(), Value::I32(114));
    item.insert("tid".to_owned(), Value::String("professions".to_owned()));
    item.insert("id".to_owned(), Value::I32(1110));
    cil.push(Value::Object(item.clone()));
    item.insert("tid".to_owned(), Value::String("grdnr".to_owned()));
    item.insert("id".to_owned(), Value::I32(1111));
    cil.push(Value::Object(item.clone()));
    item.insert("tid".to_owned(), Value::String("jntr".to_owned()));
    item.insert("id".to_owned(), Value::I32(1112));
    cil.push(Value::Object(item.clone()));
    item.insert("tid".to_owned(), Value::String("vsgst".to_owned()));
    item.insert("id".to_owned(), Value::I32(1577));
    cil.push(Value::Object(item.clone()));
    item.insert("tid".to_owned(), Value::String("phtghr".to_owned()));
    item.insert("id".to_owned(), Value::I32(1578));
    cil.push(Value::Object(item));
    out.insert("cil".to_owned(), Value::Vector(cil));
    return Value::Object(out);
}

fn chat_decor_campaign() -> Value {
    let mut out = HashMap::new();
    out.insert("st".to_owned(), Value::I32(1));
    out.insert("v".to_owned(), Value::I32(1));
    out.insert("id".to_owned(), Value::I32(449));
    out.insert("iu".to_owned(), Value::String("".to_owned()));
    out.insert("tp".to_owned(), Value::I32(9));
    out.insert("ed".to_owned(), Value::Date(1522616400000));
    let mut cil = Vec::new();
    let mut item = HashMap::new();
    item.insert("sc".to_owned(), Value::I32(0));
    item.insert("gl".to_owned(), Value::I32(0));
    item.insert("si".to_owned(), Value::I32(0));
    item.insert("cid".to_owned(), Value::I32(449));
    item.insert("tid".to_owned(), Value::String("chatDecor".to_owned()));
    item.insert("id".to_owned(), Value::I32(4095));
    cil.push(Value::Object(item));
    out.insert("cil".to_owned(), Value::Vector(cil));
    return Value::Object(out);
}

fn chat_decor_shop_campaign() -> Value {
    let mut out = HashMap::new();
    out.insert("st".to_owned(), Value::I32(1));
    out.insert("v".to_owned(), Value::I32(1));
    out.insert("id".to_owned(), Value::I32(457));
    out.insert("iu".to_owned(), Value::String("".to_owned()));
    out.insert("tp".to_owned(), Value::I32(9));
    out.insert("ed".to_owned(), Value::Date(1530133200000));
    let mut cil = Vec::new();
    let mut item = HashMap::new();
    item.insert("sc".to_owned(), Value::I32(0));
    item.insert("gl".to_owned(), Value::I32(0));
    item.insert("si".to_owned(), Value::I32(0));
    item.insert("cid".to_owned(), Value::I32(457));
    item.insert("tid".to_owned(), Value::String("chatDecorShop".to_owned()));
    item.insert("id".to_owned(), Value::I32(4220));
    cil.push(Value::Object(item));
    out.insert("cil".to_owned(), Value::Vector(cil));
    return Value::Object(out);
}
