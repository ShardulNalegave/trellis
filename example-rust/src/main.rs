
// ===== Imports =====
use std::env;
use std::io::Error;
use trellis::prelude::*;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let run_type = args[1].clone();
  let host: (u8, u8, u8, u8) = (args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap());
  let port: u16 = args[6].parse().unwrap();
  let address = Address::new(host, port);

  if run_type == "lis" {
    let listener = Listener::new(address).await?;
    println!("Listening at: {:?}", listener.local_addr);
    loop {
      let mut conn = listener.accept().await?;
      println!("New Connection: {:?}", conn.peer_addr);
      conn.write_u8(100).await?;
      conn.write_string(String::from("Hello, World!")).await?;
    }
  } else if run_type == "client" {
    let mut conn = Connection::new(address).await?;
    println!("Connected to: {:?}", conn.peer_addr);
    println!("Local Address: {:?}", conn.local_addr);
    let num = conn.read_u8().await?;
    println!("Message Received: {:?}", num);
    let data = conn.read_string(13).await?;
    println!("{:?}", data);
  }

  Ok(())
}