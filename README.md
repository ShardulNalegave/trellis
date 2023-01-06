
<div style="text-align: center;">
    <h1>Trellis</h1>
    <p>A simple networking library for Rust-Lang.</p>
</div>

----

## Getting Started

**Listener:-**
````rust

// ===== Imports =====
use std::io::Error;
use trellis_net::prelude::*;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let address = Address::new(
    (127, 0, 0, 1),
    8080,
  );
  
  let listener = Listener::new(address).await?;
  println!("Listening at: {:?}", listener.local_addr);
  loop {
    let mut conn = listener.accept().await?;
    println!("New Connection: {:?}", conn.peer_addr);
    conn.write_u8(100).await?;
    conn.write_string(String::from("Hello, World!")).await?;
  }
  Ok(())
}

````

**Client:-**
````rust

// ===== Imports =====
use std::io::Error;
use trellis_net::prelude::*;
// ===================

#[tokio::main]
async fn main() -> Result<(), Error> {
  let address = Address::new(
    (127, 0, 0, 1),
    8080,
  );
  
  let mut conn = Connection::new(address).await?;
  println!("Connected to: {:?}", conn.peer_addr);
  println!("Local Address: {:?}", conn.local_addr);
  let num = conn.read_u8().await?;
  println!("Message Received: {:?}", num);
  let data = conn.read_string(13).await?;
  println!("{:?}", data);
  Ok(())
}

````