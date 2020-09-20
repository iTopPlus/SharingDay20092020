use nats;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() -> std::io::Result<()> {
    match pubish() {
        Ok(()) => {
            println!("Done")
        }
        Err(e) =>{
            println!("Error : {}", e);
        }
    }
    Ok(())
}

fn pubish() -> std::io::Result<()> {
    let nc = nats::connect("localhost")?;
    nc.publish("my.subject", "Hello World!")?;
    Ok(())
}
