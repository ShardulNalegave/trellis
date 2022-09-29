
// ===== Imports =====
use std::{io::Error, net::SocketAddr, collections::HashMap};
use bytes::BytesMut;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

use crate::{
  Address,
  rw::{ReadPreReqs, WritePreReqs},
};
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

#[async_trait]
impl ReadPreReqs for Connection {
  async fn read(&mut self, byts: &mut BytesMut) -> Result<(), Error> {
    self.stream.read(byts).await?;
    Ok(())
  }
}

#[async_trait]
impl WritePreReqs for Connection {
  async fn write(&mut self, byts: &mut BytesMut) -> Result<(), Error> {
    self.stream.write(byts).await?;
    Ok(())
  }
}