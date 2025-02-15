use anchor_lang::prelude::*;
use borsh::{
  maybestd::io::{Error, ErrorKind, Result, Write},
  BorshDeserialize, BorshSerialize,
};
use core::ops::{Add, Mul, Sub};
use curve25519_dalek::{
  constants::{RISTRETTO_BASEPOINT_COMPRESSED, RISTRETTO_BASEPOINT_POINT},
  ristretto::{CompressedRistretto, RistrettoPoint},
  scalar::Scalar,
};
use lazy_static::lazy_static;
use sha3::Sha3_512;

lazy_static! {
  /// Pedersen base point for encoding messages to be committed.
  pub static ref G: RistrettoPoint = RISTRETTO_BASEPOINT_POINT;
  /// Pedersen base point for encoding the commitment openings.
  pub static ref H: RistrettoPoint =
      RistrettoPoint::hash_from_bytes::<Sha3_512>(RISTRETTO_BASEPOINT_COMPRESSED.as_bytes());
}

/**
 * EC Point
 */
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point {
  value: RistrettoPoint,
}

impl BorshSerialize for Point {
  fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
    let bytes = self.value.compress().to_bytes();
    writer.write_all(&bytes)
  }
}

impl BorshDeserialize for Point {
  fn deserialize(buf: &mut &[u8]) -> Result<Self> {
    Ok(Point {
      value: CompressedRistretto::from_slice(buf)
        .decompress()
        .ok_or_else(|| {
          Error::new(
            ErrorKind::InvalidData,
            "Cannot serialize/deserialize the struct",
          )
        })?,
    })
  }
}

impl Point {
  pub const G: RistrettoPoint = RISTRETTO_BASEPOINT_POINT;

  pub fn add(&self, other: Point) -> Point {
    let p = self.value.add(other.value);
    return Point { value: p };
  }

  pub fn sub(&self, other: Point) -> Point {
    let p = self.value.sub(other.value);
    return Point { value: p };
  }

  pub fn mul(&self, scalar: Scalar) -> Point {
    let p = self.value.mul(scalar);
    return Point { value: p };
  }
}
