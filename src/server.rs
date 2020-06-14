use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use crate::client::Client;
use crate::common::PlayerData;
use crate::modules;

pub struct Server {
    pub modules: Arc<RwLock<HashMap<String, Box<dyn modules::Base>>>>,
    pub player_data: Arc<RwLock<HashMap<String, PlayerData>>>
}

impl Server {
    pub fn listen(self) {
        let listener = TcpListener::bind("0.0.0.0:8123").unwrap();
        for stream in listener.incoming() {
            println!("new connection");
            let modules = Arc::clone(&self.modules);
            let player_data = Arc::clone(&self.player_data);
            thread::spawn(move || {
                let mut client = Client::new(stream.unwrap(), modules, player_data);
                client.handle();
            });
        }
    }

    pub fn new() -> Server {
        let player_data = Arc::new(RwLock::new(HashMap::new()));
        let modules: Arc<RwLock<HashMap<String, Box<dyn modules::Base>>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut lock = modules.write().unwrap();
        let module = modules::house::House::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::avatar::Avatar::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::billing::Billing::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::outside::Outside::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::component::Component::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::descriptor::Descriptor::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::furniture::Furniture::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::passport::Passport::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::player::Player::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        let module = modules::chat_decor::ChatDecor::new();
        lock.insert(module.prefix.to_owned(), Box::new(module));
        drop(lock);
        Server {
            modules,
            player_data
        }
    }
}
