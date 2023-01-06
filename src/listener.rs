
// ===== Imports =====
use std::{io::Error, net::SocketAddr};
use tokio::net::TcpListener;

use crate::{Address, connection::Connection};
// ===================

/// # Listener
/// I/O Object for listening to and accepting `Connection`s.
pub struct Listener {
  pub local_addr: Address,
  listener: TcpListener,
}

impl Listener {
  /// ## Constructor
  /// Creates a new `Listener` instance.
  pub async fn new(listen_at: Address) -> Result<Self, Error> {
    let listen_at: SocketAddr = listen_at.into();
    let listener = TcpListener::bind(listen_at).await?;
    let local_addr = listener.local_addr()?.into();
    Ok(Self { local_addr, listener })
  }

  /// ## Accept
  /// Accepts an incoming `Connection`.
  pub async fn accept(&self) -> Result<Connection, Error> {
    let (stream, _) = self.listener.accept().await?;
    let conn = Connection::try_from(stream)?;
    Ok(conn)
  }
}