//! Finite fields.
//!
//! This module defines a few specific finite fields, used as base fields
//! by various curves. These are merely specializations of the
//! backend-provided `GF255` and `ModInt256` types.

pub use crate::backend::{GF255, ModInt256, GFsecp256k1};

/// Field: integers modulo 2^255 - 19
/// (base field for Curve25519 and derivatives: X25519, ed25519, Ristretto255).
pub type GF25519 = GF255<19>;

/// Field: integers modulo 2^255 - 18651
/// (base field for double-odd curve do255e).
pub type GF255e = GF255<18651>;

/// Field: integers modulo 2^255 - 3957
/// (base field for double-odd curve do255s).
pub type GF255s = GF255<3957>;

/// Field: integers modulo 2^256 - 2^224 + 2^192 + 2^96 - 1
/// (base field for NIST curve P-256).
pub type GFp256 = ModInt256<0xFFFFFFFFFFFFFFFF, 0x00000000FFFFFFFF,
                            0x0000000000000000, 0xFFFFFFFF00000001>;

impl GF25519 {
    /// Field element encoding length (in bytes).
    pub const ENC_LEN: usize = 32;

    /// Encodes a field element into bytes (little-endian).
    pub fn encode(self) -> [u8; 32] {
        self.encode32()
    }
}

impl GF255e {
    /// Field element encoding length (in bytes).
    pub const ENC_LEN: usize = 32;

    /// Encodes a field element into bytes (little-endian).
    pub fn encode(self) -> [u8; 32] {
        self.encode32()
    }
}

impl GF255s {
    /// Field element encoding length (in bytes).
    pub const ENC_LEN: usize = 32;

    /// Encodes a field element into bytes (little-endian).
    pub fn encode(self) -> [u8; 32] {
        self.encode32()
    }
}

impl GFp256 {
    /// Field element encoding length (in bytes).
    pub const ENC_LEN: usize = 32;

    /// Encodes a field element into bytes (little-endian).
    pub fn encode(self) -> [u8; 32] {
        self.encode32()
    }
}

impl GFsecp256k1 {
    /// Field element encoding length (in bytes).
    pub const ENC_LEN: usize = 32;

    /// Encodes a field element into bytes (little-endian).
    pub fn encode(self) -> [u8; 32] {
        self.encode32()
    }
}
