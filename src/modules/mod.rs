use bytes::{BytesMut, BufMut};
use crc::{crc32, Hasher32};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Mutex, Arc};
use std::io::Write;
use std::net::TcpStream;
use redis::Commands;
use crate::encoder;
use crate::client::Client;
use crate::common::{PlayerData, Value};
use crate::inventory;
pub mod location;
pub mod house;
pub mod outside;
pub mod avatar;
pub mod billing;
pub mod notify;
pub mod component;
pub mod descriptor;
pub mod campaign;
pub mod furniture;
pub mod passport;
pub mod player;
pub mod chat_decor;

pub trait Base: Send+Sync {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>>;
}

pub fn get_appearance(uid: &str, redis: &redis::Client) -> Result<Option<HashMap<String, Value>>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let apprnc: Option<Vec<String>> = con.lrange(format!("uid:{}:appearance", uid), 0, -1)?;
    match apprnc {
        Some(vec) => {
            let mut out: HashMap<String, Value> = HashMap::new();
            if vec.len() == 0 {
                return Ok(None);
            }
            out.insert("n".to_owned(), Value::String(vec[0].clone()));          // name
            out.insert("nct".to_owned(), Value::I32(vec[1].parse::<i32>()?));   // name change time
            out.insert("g".to_owned(), Value::I32(vec[2].parse::<i32>()?));     // gender
            out.insert("sc".to_owned(), Value::I32(vec[3].parse::<i32>()?));    // skin color
            out.insert("ht".to_owned(), Value::I32(vec[4].parse::<i32>()?));    // hair type
            out.insert("hc".to_owned(), Value::I32(vec[5].parse::<i32>()?));    // hair color
            out.insert("brt".to_owned(), Value::I32(vec[6].parse::<i32>()?));   // brows type
            out.insert("brc".to_owned(), Value::I32(vec[7].parse::<i32>()?));   // brows color
            out.insert("et".to_owned(), Value::I32(vec[8].parse::<i32>()?));    // eyes type
            out.insert("ec".to_owned(), Value::I32(vec[9].parse::<i32>()?));    // eyes color
            out.insert("fft".to_owned(), Value::I32(vec[10].parse::<i32>()?));  // face feature type
            out.insert("fat".to_owned(), Value::I32(vec[11].parse::<i32>()?));  // face art type
            out.insert("fac".to_owned(), Value::I32(vec[12].parse::<i32>()?));  // face art color
            out.insert("ss".to_owned(), Value::I32(vec[13].parse::<i32>()?));   // strass type
            out.insert("ssc".to_owned(), Value::I32(vec[14].parse::<i32>()?));  // strass color
            out.insert("mt".to_owned(), Value::I32(vec[15].parse::<i32>()?));   // mouth type
            out.insert("mc".to_owned(), Value::I32(vec[16].parse::<i32>()?));   // mouth color
            out.insert("sh".to_owned(), Value::I32(vec[17].parse::<i32>()?));   // shadow type
            out.insert("shc".to_owned(), Value::I32(vec[18].parse::<i32>()?));  // shadow color
            out.insert("rg".to_owned(), Value::I32(vec[19].parse::<i32>()?));   // rouge type
            out.insert("rc".to_owned(), Value::I32(vec[20].parse::<i32>()?));   // rouge color
            out.insert("pt".to_owned(), Value::I32(vec[21].parse::<i32>()?));   // pirsing type
            out.insert("pc".to_owned(), Value::I32(vec[22].parse::<i32>()?));   // pirsing color
            out.insert("bt".to_owned(), Value::I32(vec[23].parse::<i32>()?));   // beard type
            out.insert("bc".to_owned(), Value::I32(vec[24].parse::<i32>()?));   // beard color
            Ok(Some(out))
        }
        None => Ok(None)
    }
}

pub fn get_plr(uid: &str, player_data: &HashMap<String, PlayerData>,
               redis: &redis::Client) -> Result<Option<HashMap<String, Value>>, Box<dyn Error>> {
    let apprnc: HashMap<String, Value>;
    let tmp = get_appearance(uid, redis)?;
    match tmp {
        Some(hashmap) => apprnc = hashmap,
        None => return Ok(None)
    }
    let mut con = redis.get_connection()?;
    let mut plr = HashMap::new();
    plr.insert("uid".to_owned(), Value::String(uid.to_owned()));
    plr.insert("apprnc".to_owned(), Value::Object(apprnc));
    plr.insert("clths".to_owned(), Value::Object(inventory::get_clths(uid, redis)?));
    plr.insert("locinfo".to_owned(), get_location(uid, player_data));
    plr.insert("chtdcm".to_owned(), get_chat_decor(uid, redis)?);
    let role: i32 = con.get(format!("uid:{}:role", uid)).unwrap_or(0);
    let mut usrinf = HashMap::new();
    usrinf.insert("rl".to_owned(), Value::I32(role));
    plr.insert("usrinf".to_owned(), Value::Object(usrinf));
    let mut professions = HashMap::new();
    for item in &["jntr", "phtghr", "grdnr", "vsgst"] {
        let mut out = HashMap::new();
        out.insert("tp".to_owned(), Value::String(item.to_string()));
        out.insert("l".to_owned(), Value::I32(20));
        out.insert("pgs".to_owned(), Value::I32(0));
        professions.insert(item.to_string(), Value::Object(out));
    }
    let mut pf = HashMap::new();
    pf.insert("pf".to_owned(), Value::Object(professions));
    plr.insert("pf".to_owned(), Value::Object(pf));
    plr.insert("ci".to_owned(), Value::Object(get_city_info(uid, redis)?));
    return Ok(Some(plr));
}

pub fn get_city_info(uid: &str, redis: &redis::Client) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let mut ci = HashMap::new();
    let exp: i32 = con.get(format!("uid:{}:exp", uid)).unwrap_or(0);
    let crt: i32 = con.get(format!("uid:{}:crt", uid)).unwrap_or(0);
    let hrt: i32 = con.get(format!("uid:{}:hrt", uid)).unwrap_or(0);
    let lvt: i32 = con.get(format!("uid:{}:lvt", uid)).unwrap_or(0);
    let trid: Option<String> = con.get(format!("uid:{}:trid", uid))?;
    ci.insert("exp".to_owned(), Value::I32(exp));                  // exp
    ci.insert("crt".to_owned(), Value::I32(crt));                  // clothes rating
    ci.insert("hrt".to_owned(), Value::I32(hrt));                  // house rating
    ci.insert("fexp".to_owned(), Value::I32(0));                   // fight exp
    ci.insert("gdc".to_owned(), Value::I32(0));                    // gift day count
    ci.insert("lgt".to_owned(), Value::I32(0));                    // last gift time
    ci.insert("vip".to_owned(), Value::Boolean(true));             // vip
    ci.insert("vexp".to_owned(), Value::I32(0));                   // vip expired at
    ci.insert("vsexp".to_owned(), Value::I32(0));                  // vip subscription expired at
    ci.insert("vsact".to_owned(), Value::Boolean(true));           // vip subscription active
    ci.insert("vret".to_owned(), Value::I32(0));                   // vip refill energy time
    ci.insert("vfgc".to_owned(), Value::I32(0));                   // vip free gifts count
    ci.insert("ceid".to_owned(), Value::I32(0));                   // co engaged id
    ci.insert("cmid".to_owned(), Value::I32(0));                   // co married id
    ci.insert("dr".to_owned(), Value::Boolean(true));              // display relations
    ci.insert("spp".to_owned(), Value::I32(0));                    // spent points
    ci.insert("tts".to_owned(), Value::None);                      // tutorial step
    ci.insert("eml".to_owned(), Value::None);                      // email
    ci.insert("ys".to_owned(), Value::I32(0));                     // yandex status
    ci.insert("ysct".to_owned(), Value::I32(0));                   // yandex session complete time
    ci.insert("fak".to_owned(), Value::None);                      // first april key
    ci.insert("shcr".to_owned(), Value::Boolean(true));            // show crown
    ci.insert("gtrfrd".to_owned(), Value::I32(0));                 // gold transferred
    ci.insert("strfrd".to_owned(), Value::I32(0));                 // silver transferred
    ci.insert("rtrtm".to_owned(), Value::I32(0));                  // resources transfer time
    ci.insert("kyktid".to_owned(), Value::None);                   // ticket id
    ci.insert("actrt".to_owned(), Value::I32(0));                  // activity rating
    ci.insert("compid".to_owned(), Value::I32(0));                 // competition id
    ci.insert("actrp".to_owned(), Value::I32(0));                  // acitivity winner place
    ci.insert("actrd".to_owned(), Value::I32(0));                  // acitivity winner until
    ci.insert("shousd".to_owned(), Value::Boolean(false));         // shared object used
    ci.insert("rpt".to_owned(), Value::I32(0));                    // reputation
    ci.insert("as".to_owned(), Value::None);                       // avamen style
    ci.insert("lvt".to_owned(), Value::I32(lvt));                  // last visit time
    ci.insert("lrnt".to_owned(), Value::I32(0));                   // last rename time
    ci.insert("lwts".to_owned(), Value::I32(0));                   // last wedding event time
    ci.insert("skid".to_owned(), Value::None);                     // skate type id
    ci.insert("skrt".to_owned(), Value::I32(0));                   // finish skate rent time
    ci.insert("bcld".to_owned(), Value::I32(0));                   // baby cooldown
    match trid {                                                   // trophy type id
        Some(v) => ci.insert("trid".to_owned(), Value::String(v.clone())),
        None => ci.insert("trid".to_owned(), Value::None)
    };
    ci.insert("trcd".to_owned(), Value::I32(0));                   // trophy cooldown
    ci.insert("sbid".to_owned(), Value::None);                     // snowboard type id
    ci.insert("sbrt".to_owned(), Value::I32(0));                   // snowboard finish rent time
    ci.insert("plcmt".to_owned(), Value::Object(HashMap::new()));  // player competition
    let mut pamns = HashMap::new();
    pamns.insert("amn".to_owned(), Value::Vector(Vec::new()));
    pamns.insert("crst".to_owned(), Value::I32(0));
    ci.insert("pamns".to_owned(), Value::Object(pamns));           // personal animations
    return Ok(ci);
}

pub fn get_location(uid: &str, player_data: &HashMap<String, PlayerData>) -> Value {
    let player = player_data.get(uid);
    match player {
        Some(v) => {
            let mut locinfo = HashMap::new();
            locinfo.insert("x".to_owned(), Value::F64(v.position[0].clone()));
            locinfo.insert("y".to_owned(), Value::F64(v.position[1].clone()));
            locinfo.insert("d".to_owned(), Value::I32(v.direction.clone()));
            locinfo.insert("st".to_owned(), Value::I32(v.state.clone()));
            locinfo.insert("at".to_owned(), Value::String(v.action_tag.clone()));
            locinfo.insert("l".to_owned(), Value::String(v.room.clone()));
            locinfo.insert("pl".to_owned(), Value::String("".to_owned()));
            locinfo.insert("s".to_owned(), Value::String("127.0.0.1".to_owned()));
            locinfo.insert("shlc".to_owned(), Value::Boolean(true));
            return Value::Object(locinfo);
        },
        None => {
            return Value::None
        }
    }
}

pub fn get_chat_decor(uid: &str, redis: &redis::Client) -> Result<Value, Box<dyn Error>> {
    let mut con = redis.get_connection()?;
    let bubble: Option<String> = con.get(format!("uid:{}:bubble", uid))?;
    let text_color: Option<String> = con.get(format!("uid:{}:text_color", uid))?;
    let mut chtdc = HashMap::new();
    match bubble {
        Some(v) => chtdc.insert("bdc".to_owned(), Value::String(v)),
        None => chtdc.insert("bdc".to_owned(), Value::None),
    };
    match text_color {
        Some(v) => chtdc.insert("tcl".to_owned(), Value::String(v)),
        None => chtdc.insert("tcl".to_owned(), Value::None),
    };
    chtdc.insert("spks".to_owned(), Value::None);
    return Ok(Value::Object(chtdc))
}

// костыль
pub fn send_to(stream: &Arc<Mutex<TcpStream>>, msg: &Vec<Value>, type_: u8) -> Result<(), Box<dyn Error>> {
    println!("send - {:?}", msg);
    let data = encoder::encode(msg, type_).unwrap();
    let length = data.len() as i32 + 5;
    let mut mask = 0;
    let mut buf = BytesMut::new();
    let checksum: u32;
    mask = mask | (1 << 3);
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(&data[..]);
    checksum = digest.sum32();
    buf.put_i32(length);
    buf.put_u8(mask);
    buf.put_u32(checksum);
    buf.extend(&data[..]);
    let mut lock = stream.lock().unwrap();
    lock.write(&buf[..])?;
    Ok(())
}
