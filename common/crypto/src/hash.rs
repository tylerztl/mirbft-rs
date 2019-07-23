use tiny_keccak::sha3_256;

pub type Digest = [u8; 32];

/// Returns the SHA-256 hash of the value's `[u8]` representation.
pub fn hash<T: AsRef<[u8]>>(value: T) -> Digest {
    sha3_256(value.as_ref())
}
