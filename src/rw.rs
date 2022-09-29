
// ===== Imports =====
use std::io::Result;
use bytes::BytesMut;
// ===================

#[async_trait]
pub trait ReadPreReqs {
  async fn read(&mut self, byts: &mut BytesMut) -> Result<()>;
}

#[async_trait]
pub trait Reader: ReadPreReqs {
  //
}

// =============================================================

#[async_trait]
pub trait WritePreReqs {
  async fn write(&mut self, byts: &mut BytesMut) -> Result<()>;
}

#[async_trait]
pub trait Writer: WritePreReqs {
  //
}