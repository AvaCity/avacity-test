use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use crate::client;
use crate::modules::{Base, house};

pub struct Server {
    pub online: Arc<Mutex<HashMap<String, client::Client>>>,
    pub modules: Arc<Mutex<HashMap<String, Box<dyn Base>>>>
}

impl Server {
    pub fn listen(self) {
        let listener = TcpListener::bind("0.0.0.0:8123").unwrap();
        for stream in listener.incoming() {
            println!("new connection");
            let online = Arc::clone(&self.online);
            let modules = Arc::clone(&self.modules);
            thread::spawn(move || {
                let mut client = client::Client::new(stream.unwrap(), online, modules);
                client.handle();
            });
        }
    }

    pub fn new() -> Server {
        let loaded_modules: Arc<Mutex<HashMap<String, Box<dyn Base>>>> = Arc::new(Mutex::new(HashMap::new()));
        let mut lock = loaded_modules.lock().unwrap();
        let online = Arc::new(Mutex::new(HashMap::new()));
        let module = house::House::new(Arc::clone(&online));
        lock.insert(module.prefix.to_owned(), Box::new(module));
        drop(lock);
        Server {
            online: online,
            modules: loaded_modules
        }
    }
}
