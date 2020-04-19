use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;
use crate::client;

pub struct Server {
    pub online: Arc<Mutex<HashMap<String, client::Client>>>,
}

impl Server {
    pub fn listen(self) {
        let listener = TcpListener::bind("0.0.0.0:8123").unwrap();
        for stream in listener.incoming() {
            println!("new connection");
            let online = Arc::clone(&self.online);
            thread::spawn(|| {
                let mut client = client::Client::new(stream.unwrap(), online);
                client.handle();
            });
        }
    }

    pub fn new() -> Server {
        Server {
            online: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}
