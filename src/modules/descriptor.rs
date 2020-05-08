use std::error::Error;
use std::collections::HashMap;
use crate::client::Client;
use crate::common::Value;
use crate::modules::Base;

pub struct Descriptor {
    pub prefix: &'static str
}

impl Descriptor {
    pub fn new() -> Descriptor {
        Descriptor {
            prefix: "dscr"
        }
    }
    fn init(&self, client: &Client, _msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("init".to_owned(), Value::Boolean(true));
        let mut locations = Vec::new();
        locations.push(self.yard());
        locations.push(self.cafe());
        locations.push(self.club());
        locations.push(self.park());
        locations.push(self.street());
        locations.push(self.public_beach());
        locations.push(self.ballroom());
        locations.push(self.canyon());
        locations.push(self.salon());
        locations.push(self.couturier());
        locations.push(self.ski_resort());
        locations.push(self.wedding_beach());
        locations.push(self.ice_rink());
        locations.push(self.podium());
        data.insert("outsideLocations".to_owned(), Value::Vector(locations));
        let mut v = Vec::new();
        v.push(Value::String("dscr.ldd".to_owned()));
        v.push(Value::Object(data));
        client.send(&v, 34)?;
        Ok(())
    }

    fn yard(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("yard".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("y1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("yard,landscape".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("y1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("yard_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("y1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn cafe(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("cafe".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("cf1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("cafe".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("cf1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("cafe_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("cafe1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("cf1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn club(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("club".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("cl1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("club,vip".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("cl1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("club_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(4));
        first.insert("bgs".to_owned(), Value::String("club1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("cl1e".to_owned()));
        let mut third = first.clone();
        third.insert("ml".to_owned(), Value::I32(0));
        third.insert("id".to_owned(), Value::String("v1".to_owned()));
        third.insert("vip".to_owned(), Value::Boolean(true));
        third.insert("uc".to_owned(), Value::String("vip1_map".to_owned()));
        third.insert("bgs".to_owned(), Value::String("vip1".to_owned()));
        let mut fourth = third.clone();
        fourth.insert("id".to_owned(), Value::String("v1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        rooms.push(Value::Object(third));
        rooms.push(Value::Object(fourth));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn park(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("park".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("p1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("park,landscape".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("p1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("park_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("p1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn street(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("street".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("s1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("street,landscape".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("s1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("street_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("s1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn public_beach(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("publicBeach".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("pb1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("publicBeach".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("pb1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("street_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("pb1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn ballroom(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("ballroom".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("br1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("ballroom".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("br1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("street_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("br1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn canyon(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("canyon".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("pc1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("canyon".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("pc1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("canyon_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn salon(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("salon".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("sn1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("salon".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("br1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("salon_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(6));
        first.insert("bgs".to_owned(), Value::String("cafe1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn couturier(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("couturier".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("ctr1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("couturier".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("ctr1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("street_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn ski_resort(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("skiResort".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("sr1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("skiResort".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("sr1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("ski_resort_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("skiResort".to_owned()));
        let mut second = first.clone();
        second.insert("id".to_owned(), Value::String("sr1e".to_owned()));
        rooms.push(Value::Object(first));
        rooms.push(Value::Object(second));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn wedding_beach(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("weddingBeach".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("wb1e".to_owned()));
        out.insert("ldc".to_owned(), Value::String("weddingBeach".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("wb1e".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("wedding_beach_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("beach".to_owned()));
        first.insert("ml".to_owned(), Value::I32(0));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn ice_rink(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("iceRink".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("ir1".to_owned()));
        out.insert("ldc".to_owned(), Value::String("iceRink".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("ir1".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("iceRink_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("outside".to_owned()));
        first.insert("ml".to_owned(), Value::I32(15));
        first.insert("bgs".to_owned(), Value::String("outside1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }

    fn podium(&self) -> Value {
        let mut out = HashMap::new();
        out.insert("id".to_owned(), Value::String("podium".to_owned()));
        out.insert("zid".to_owned(), Value::String("street".to_owned()));
        out.insert("drid".to_owned(), Value::String("pdm".to_owned()));
        out.insert("ldc".to_owned(), Value::String("podium,iceRink".to_owned()));
        let mut rooms = Vec::new();
        let mut first = HashMap::new();
        first.insert("id".to_owned(), Value::String("pdm".to_owned()));
        first.insert("vip".to_owned(), Value::Boolean(false));
        first.insert("uc".to_owned(), Value::String("podium_1_map".to_owned()));
        first.insert("dc".to_owned(), Value::String("beach".to_owned()));
        first.insert("ml".to_owned(), Value::I32(15));
        first.insert("bgs".to_owned(), Value::String("Podium1".to_owned()));
        rooms.push(Value::Object(first));
        out.insert("rms".to_owned(), Value::Vector(rooms));
        return Value::Object(out)
    }
}

impl Base for Descriptor {
    fn handle(&self, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
        let tmp = msg[1].get_string()?;
        let splitted: Vec<&str> = tmp.split(".").collect();
        let command = splitted[1];
        match command {
            "init" => self.init(client, msg)?,
            _ => println!("Command {} not found", tmp)
        }
        Ok(())
    }
}
