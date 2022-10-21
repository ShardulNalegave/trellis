
// ===== Imports =====
#[macro_use] extern crate async_trait;
use std::{net::{SocketAddr, IpAddr, Ipv4Addr}};

use bytes::{BytesMut, BufMut};
// ===================

pub mod node;
pub mod listener;
pub mod connection;
pub mod rw;
pub mod prelude;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Address {
  pub host: (u8, u8, u8, u8),
  pub port: u16,
}

impl Address {
  pub fn new(host: (u8, u8, u8, u8), port: u16) -> Self {
    Self { host, port }
  }
}


impl Into<SocketAddr> for Address {
  fn into(self) -> SocketAddr {
    let host = IpAddr::V4(Ipv4Addr::new(self.host.0, self.host.1, self.host.2, self.host.3));
    SocketAddr::new(host, self.port)
  }
}

impl From<SocketAddr> for Address {
  fn from(addr: SocketAddr) -> Self {
    if let IpAddr::V4(v4_addr) = addr.ip() {
      let host_octets = v4_addr.octets();
      let host = (host_octets[0], host_octets[1], host_octets[2], host_octets[3]);
      return Self { host, port: addr.port() }
    } else {
      panic!("Expected an IpV4 Socket Address")
    }
  }
}

pub fn zero_filled_bytes_mut(len: usize) -> BytesMut {
  let mut byts = BytesMut::with_capacity(len);
  byts.put(&*vec![0; len]);
  byts
}