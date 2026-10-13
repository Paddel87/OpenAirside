//! Deterministic state hash (FNV-1a, 64 bit) for comparing simulation runs
//! bit by bit (ADR-010) and, later, for the state checksum of save games
//! (S-5, FR-004). Independent of platform, thread count and `std` hashers.

const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;

/// Incremental FNV-1a hasher over little-endian bytes.
#[derive(Debug, Clone)]
pub struct StateHasher(u64);

impl Default for StateHasher {
    fn default() -> Self {
        Self(OFFSET)
    }
}

impl StateHasher {
    /// Fresh hasher.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds raw bytes.
    pub fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(PRIME);
        }
    }

    /// Feeds a `u32`.
    pub fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    /// Feeds the bit pattern of an `f32`.
    pub fn f32(&mut self, value: f32) {
        self.u32(value.to_bits());
    }

    /// Final hash value.
    #[must_use]
    pub fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_fnv1a_reference_values() {
        assert_eq!(StateHasher::new().finish(), 0xcbf2_9ce4_8422_2325);
        let mut h = StateHasher::new();
        h.bytes(b"a");
        assert_eq!(h.finish(), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn distinguishes_float_bit_patterns() {
        let hash = |v: f32| {
            let mut h = StateHasher::new();
            h.f32(v);
            h.finish()
        };
        assert_ne!(hash(0.0), hash(-0.0));
        assert_eq!(hash(1.5), hash(1.5));
    }
}
