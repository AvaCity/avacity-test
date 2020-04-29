use std::collections::HashMap;
use std::error::Error;
use crate::client::Client;
use crate::common::Value;
use crate::modules::send_to;


pub fn join_room(prefix: &'static str, client: &Client, room: &str) -> Result<(), Box<dyn Error>> {
    let mut player_data = client.player_data.lock().unwrap();
    let mut current_player = player_data.get_mut(&client.uid).ok_or("Can't get mut")?;
    current_player.room = room.to_owned();
    current_player.position = [-1.0, -1.0];
    current_player.direction = 4;
    current_player.state = 0;
    current_player.action_tag = "".to_owned();
    let mut msg1 = Vec::new();
    msg1.push(Value::String(format!("{}.r.jn", prefix)));
    let mut out_data = HashMap::new();
    out_data.insert("uid".to_owned(), Value::String(client.uid.clone()));
    msg1.push(Value::Object(out_data));
    let mut msg2 = Vec::new();
    msg2.push(Value::String(room.to_owned()));
    msg2.push(Value::String(client.uid.clone()));
    for player_uid in player_data.keys() {
        let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
        if &player.room == room {
            send_to(&player.stream, &msg1, 34)?;
            send_to(&player.stream, &msg2, 16)?;
        }
    }
    Ok(())
}

pub fn leave_room(prefix: &'static str, client: &Client) -> Result<(), Box<dyn Error>> {
    let mut player_data = client.player_data.lock().unwrap();
    let mut current_player = player_data.get_mut(&client.uid).ok_or("Can't get mut")?;
    if current_player.room.is_empty() {
        return Ok(())
    }
    let room = current_player.room.clone();
    current_player.room = "".to_owned();
    let mut msg1 = Vec::new();
    msg1.push(Value::String(format!("{}.r.lv", prefix)));
    let mut out_data = HashMap::new();
    out_data.insert("uid".to_owned(), Value::String(client.uid.clone()));
    msg1.push(Value::Object(out_data));
    let mut msg2 = Vec::new();
    msg2.push(Value::String(room.to_owned()));
    msg2.push(Value::String(client.uid.clone()));
    for player_uid in player_data.keys() {
        let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
        if player.room == room {
            send_to(&player.stream, &msg1, 34)?;
            send_to(&player.stream, &msg2, 17)?;
        }
    }
    Ok(())
}

pub fn room(prefix: &'static str, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    let tmp = msg[1].get_string()?;
    let splitted: Vec<&str> = tmp.split(".").collect();
    let command = splitted[2];
    match command {
        "u" => update_state(prefix, client, msg)?,
        _ => println!("Command {} not found", tmp)
    }
    Ok(())
}

fn update_state(prefix: &'static str, client: &Client, msg: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    let data = msg[2].get_object()?;
    let room_name = msg[0].get_string()?;
    let uid = data.get("uid").ok_or("err")?.get_string()?;
    if uid != client.uid {
        println!("{} tried to fake his uid", &client.uid);
        return Ok(())
    }
    let x = data.get("x").ok_or("err")?.get_f64()?;
    let y = data.get("x").ok_or("err")?.get_f64()?;
    let direction = data.get("d").ok_or("err")?.get_i32()?;
    let state = data.get("st").ok_or("err")?.get_i32()?;
    let action_tag: String;
    if data.contains_key("at") {
        action_tag = data.get("at").unwrap().get_string()?;
    }
    else {
        action_tag = "".to_owned();
    }
    let mut player_data = client.player_data.lock().unwrap();
    let mut current_player = player_data.get_mut(&client.uid).ok_or("Can't get mut")?;
    current_player.position = [x, y];
    current_player.direction = direction;
    current_player.state = state;
    current_player.action_tag = action_tag;
    let mut v = Vec::new();
    v.push(Value::String(format!("{}.r.u", prefix)));
    v.push(msg[2].clone());
    for player_uid in player_data.keys() {
        if player_uid == &client.uid {
            continue;
        }
        let player = player_data.get(&player_uid.clone()).ok_or("player not found")?;
        if player.room == room_name {
            send_to(&player.stream, &v, 34)?;
        }
    }
    Ok(())
}
