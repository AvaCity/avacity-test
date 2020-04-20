extern crate hex;
extern crate redis;
use std::collections::HashMap;
use std::io::{Read, Write, Cursor};
use std::sync::{Mutex, Arc};
use std::net::{TcpStream, Shutdown};
use bytes::{BytesMut, BufMut};
use crc::{crc32, Hasher32};
use redis::Commands;
use crate::common;
use crate::decoder;
use crate::encoder;
use crate::base_messages;
use crate::modules::Base;

static XML: &'static str = "<?xml version=\"1.0\"?>
<cross-domain-policy>
<allow-access-from domain=\"*\" to-ports=\"*\" />
</cross-domain-policy>";
static EMPTY: &'static [u8] = &[0, 0];

pub struct Client {
    pub stream: TcpStream,
    pub uid: String,
    pub online: Arc<Mutex<HashMap<String, Client>>>,
    modules: Arc<Mutex<HashMap<String, Box<dyn Base>>>>,
    redis: redis::Client,
    pub encrypted: bool,
    pub compressed: bool,
    pub checksummed: bool,
}

impl Client {
    pub fn handle(&mut self) {
        let mut buffer = [0 as u8; 1024];
        loop {
            let size = self.stream.read(&mut buffer).unwrap();
            let hex_string = hex::encode(&buffer[..size]);
            if size == 0 {
                self.stream.shutdown(Shutdown::Both).expect("Shutdown failed!");
                break;
            }
            if hex_string == "3c706f6c6963792d66696c652d726571756573742f3e00" {
                let bytes = &[XML.as_bytes(), EMPTY].concat()[..];
                self.stream.write(bytes).expect("Write failed");
                self.stream.shutdown(Shutdown::Both).expect("Shutdown failed!");
                break;
            }
            let data = &buffer[..size];
            let mut cur = Cursor::new(data);
            while data.len() as i32 - cur.position() as i32 > 4 {
                let mut tmp = [0; 4];
                cur.read_exact(&mut tmp).unwrap();
                let length = i32::from_be_bytes(tmp);
                if data.len() as i32 - (cur.position() as i32) < length{
                    break;
                }
                let pos = cur.position() as usize;
                let tmp_data = &data[pos..pos+(length as usize)];
                cur.set_position(cur.position() + (length as u64));
                let message = decoder::decode(&tmp_data).unwrap();
                let type_ = message.get("type").expect("kavo").get_u8().unwrap();
                let msg = message.get("msg").expect("kavo").get_vector().unwrap();
                println!("type - {}", type_);
                println!("msg - {:?}", msg);
                if type_ == 1 {
                    self.auth(msg);
                }
                else if type_ == 34 {
                    let tmp = msg[1].get_string().unwrap();
                    let splitted: Vec<&str> = tmp.split(".").collect();
                    let module_name = splitted[0].to_owned();
                    let lock = self.modules.lock().unwrap();
                    if !lock.contains_key(&module_name) {
                        println!("Command {} not found", tmp);
                        continue;
                    }
                    let module = lock.get(&module_name).expect("Impossible");
                    module.handle(&self, msg);
                }
            }
            buffer = [0 as u8; 1024];
        }
        println!("drop connection");
    }

    pub fn send(&mut self, msg: Vec<common::Value>, type_: u8) {
        let data = encoder::encode(msg, type_).unwrap();
        let mut length = data.len() as i32 + 1;
        let mut mask = 0;
        let mut buf = BytesMut::new();
        let mut checksum: u32 = 0;
        if self.checksummed {
            mask = mask | (1 << 3);
            length = length + 4;
            let mut digest = crc32::Digest::new(crc32::IEEE);
            digest.write(&data[..]);
            checksum = digest.sum32();
        }
        buf.put_i32(length);
        buf.put_u8(mask);
        if self.checksummed {
            buf.put_u32(checksum);
        }
        buf.extend(&data[..]);
        self.stream.write(&buf[..]).unwrap();
    }

    fn auth(&mut self, msg: &Vec<common::Value>) {
        let uid = msg[1].get_string().unwrap();
        let token = msg[2].get_string().unwrap();
        let mut con = self.redis.get_connection().unwrap();
        match con.get(format!("auth:{}", token)) {
            Ok(value) => {
                let real_uid: String = value;
                if uid != real_uid {
                    let msg = base_messages::wrong_pass();
                    self.send(msg, 2);
                    self.stream.shutdown(Shutdown::Both).expect("Shutdown failed!");
                    return;
                }
                self.uid = real_uid.clone();
                let mut v: Vec<common::Value> = Vec::new();
                v.push(common::Value::String(real_uid));
                v.push(common::Value::Boolean(true));
                v.push(common::Value::Boolean(false));
                v.push(common::Value::Boolean(false));
                self.send(v, 1);
            }
            Err(_) => {
                let msg = base_messages::wrong_pass();
                self.send(msg, 2);
                self.stream.shutdown(Shutdown::Both).expect("Shutdown failed!");
                return;
            }
        }
    }

    pub fn new(stream: TcpStream, online: Arc<Mutex<HashMap<String, Client>>>,
               modules: Arc<Mutex<HashMap<String, Box<dyn Base>>>>) -> Client {
        Client {
            stream: stream,
            uid: String::from("0"),
            online: online,
            modules: modules,
            redis: redis::Client::open("redis://127.0.0.1/").unwrap(),
            checksummed: false,
            compressed: false,
            encrypted: false
        }
    }
}
