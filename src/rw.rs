
// ===== Imports =====
use std::io::Result;
use bytes::{BytesMut, Buf, BufMut};

use crate::zero_filled_bytes_mut;
// ===================

#[async_trait]
pub trait ReadPreReqs {
  async fn read(&mut self, byts: &mut BytesMut) -> Result<()>;
}

#[async_trait]
pub trait Reader: ReadPreReqs {
  async fn read_u8(&mut self) -> Result<u8> {
    let mut byts = zero_filled_bytes_mut(1);
    self.read(&mut byts).await?;
    let res = byts.get_u8();
    Ok(res)
  }

  async fn read_u16(&mut self) -> Result<u16> {
    let mut byts = zero_filled_bytes_mut(2);
    self.read(&mut byts).await?;
    let res = byts.get_u16();
    Ok(res)
  }

  async fn read_u32(&mut self) -> Result<u32> {
    let mut byts = zero_filled_bytes_mut(4);
    self.read(&mut byts).await?;
    let res = byts.get_u32();
    Ok(res)
  }

  async fn read_u64(&mut self) -> Result<u64> {
    let mut byts = zero_filled_bytes_mut(8);
    self.read(&mut byts).await?;
    let res = byts.get_u64();
    Ok(res)
  }

  async fn read_u128(&mut self) -> Result<u128> {
    let mut byts = zero_filled_bytes_mut(16);
    self.read(&mut byts).await?;
    let res = byts.get_u128();
    Ok(res)
  }

  async fn read_i8(&mut self) -> Result<i8> {
    let mut byts = zero_filled_bytes_mut(1);
    self.read(&mut byts).await?;
    let res = byts.get_i8();
    Ok(res)
  }

  async fn read_i16(&mut self) -> Result<i16> {
    let mut byts = zero_filled_bytes_mut(2);
    self.read(&mut byts).await?;
    let res = byts.get_i16();
    Ok(res)
  }

  async fn read_i32(&mut self) -> Result<i32> {
    let mut byts = zero_filled_bytes_mut(4);
    self.read(&mut byts).await?;
    let res = byts.get_i32();
    Ok(res)
  }

  async fn read_i64(&mut self) -> Result<i64> {
    let mut byts = zero_filled_bytes_mut(8);
    self.read(&mut byts).await?;
    let res = byts.get_i64();
    Ok(res)
  }

  async fn read_i128(&mut self) -> Result<i128> {
    let mut byts = zero_filled_bytes_mut(16);
    self.read(&mut byts).await?;
    let res = byts.get_i128();
    Ok(res)
  }

  async fn read_f32(&mut self) -> Result<f32> {
    let mut byts = zero_filled_bytes_mut(4);
    self.read(&mut byts).await?;
    let res = byts.get_f32();
    Ok(res)
  }

  async fn read_f64(&mut self) -> Result<f64> {
    let mut byts = zero_filled_bytes_mut(8);
    self.read(&mut byts).await?;
    let res = byts.get_f64();
    Ok(res)
  }
}

// =============================================================

#[async_trait]
pub trait WritePreReqs {
  async fn write(&mut self, byts: &mut BytesMut) -> Result<()>;
}

#[async_trait]
pub trait Writer: WritePreReqs {
  async fn write_u8(&mut self, data: u8) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_u8(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_u16(&mut self, data: u16) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_u16(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_u32(&mut self, data: u32) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_u32(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_u64(&mut self, data: u64) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_u64(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_u128(&mut self, data: u128) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_u128(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_i8(&mut self, data: i8) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_i8(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_i16(&mut self, data: i16) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_i16(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_i32(&mut self, data: i32) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_i32(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_i64(&mut self, data: i64) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_i64(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_i128(&mut self, data: i128) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_i128(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_f32(&mut self, data: f32) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_f32(data);
    self.write(&mut byts).await?;
    Ok(())
  }

  async fn write_f64(&mut self, data: f64) -> Result<()> {
    let mut byts = BytesMut::new();
    byts.put_f64(data);
    self.write(&mut byts).await?;
    Ok(())
  }
}