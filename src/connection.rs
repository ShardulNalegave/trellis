
// ===== Imports =====
use std::{io::Error, net::SocketAddr, collections::HashMap};
use tokio::net::TcpStream;

use crate::Address;
// ===================

pub type Connections = HashMap<Address, Connection>;

pub struct Connection {
  pub local_addr: Address,
  pub peer_addr: Address,
  stream: TcpStream,
}

impl Connection {
  pub async fn new(to: Address) -> Result<Self, Error> {
    let to: SocketAddr = to.into();
    let stream = TcpStream::connect(to).await?;
    let local_addr = stream.local_addr()?.into();
    let peer_addr = stream.peer_addr()?.into();
    Ok(Self { local_addr, peer_addr, stream })
  }
}

impl TryFrom<TcpStream> for Connection {
  type Error = Error;

  fn try_from(stream: TcpStream) -> Result<Self, Self::Error> {
    let local_addr = stream.local_addr()?.into();
    let peer_addr = stream.peer_addr()?.into();
    Ok(Self { local_addr, peer_addr, stream })
  }
}