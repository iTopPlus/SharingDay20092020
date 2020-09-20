use nats;
use serde::{Deserialize, Serialize};
// use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() -> std::io::Result<()> {
    match subscribe() {
        Ok(()) => {
            println!("Recieve")
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
    Ok(())
}

fn subscribe() -> std::io::Result<()> {
    let nc = nats::connect("localhost")?;
    let sub = nc.subscribe("my.subject")?;
    for msg in sub.messages() {
        println!("Message : {}", msg)
    }
    Ok(())
}