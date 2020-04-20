use crate::client::Client;
use crate::common::Value;
pub mod house;

pub trait Base: Send{
    fn handle(&self, client: &Client, msg: &Vec<Value>);
}
