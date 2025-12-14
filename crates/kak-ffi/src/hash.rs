//! Hash function implementations matching Kakoune's hash.hh/hash.cc

const FNV_PRIME_32: u32 = 16777619;
const FNV_OFFSET_BASIS_32: u32 = 2166136261;

/// FNV-1a hash function.
///
/// Matches Kakoune's `fnv1a` implementation in hash.hh.
#[unsafe(no_mangle)]
pub extern "C" fn fnv1a(data: *const u8, len: usize) -> usize {
    if data.is_null() {
        return FNV_OFFSET_BASIS_32 as usize;
    }

    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let mut hash = FNV_OFFSET_BASIS_32;

    for &byte in bytes {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME_32);
    }

    hash as usize
}

#[inline(always)]
fn rotl32(x: u32, r: i8) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[inline(always)]
fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

/// MurmurHash3 (32-bit) hash function.
///
/// Matches Kakoune's `murmur3` implementation in hash.cc.
/// Based on <https://github.com/PeterScott/murmur3>
#[unsafe(no_mangle)]
pub extern "C" fn murmur3(input: *const u8, len: usize) -> usize {
    if input.is_null() {
        return fmix32(0x1235678 ^ 0) as usize;
    }

    let data = unsafe { std::slice::from_raw_parts(input, len) };
    let mut hash: u32 = 0x1235678;
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    let nblocks = len / 4;

    for i in 0..nblocks {
        let offset = i * 4;
        let key_bytes = [
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ];
        let mut key = u32::from_le_bytes(key_bytes);

        key = key.wrapping_mul(C1);
        key = rotl32(key, 15);
        key = key.wrapping_mul(C2);

        hash ^= key;
        hash = rotl32(hash, 13);
        hash = hash.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let tail = &data[nblocks * 4..];
    let mut key: u32 = 0;

    match len & 0b11 {
        3 => {
            key ^= (tail[2] as u32) << 16;
            key ^= (tail[1] as u32) << 8;
            key ^= tail[0] as u32;
            key = key.wrapping_mul(C1);
            key = rotl32(key, 15);
            key = key.wrapping_mul(C2);
            hash ^= key;
        }
        2 => {
            key ^= (tail[1] as u32) << 8;
            key ^= tail[0] as u32;
            key = key.wrapping_mul(C1);
            key = rotl32(key, 15);
            key = key.wrapping_mul(C2);
            hash ^= key;
        }
        1 => {
            key ^= tail[0] as u32;
            key = key.wrapping_mul(C1);
            key = rotl32(key, 15);
            key = key.wrapping_mul(C2);
            hash ^= key;
        }
        _ => {}
    }

    hash ^= len as u32;
    hash = fmix32(hash);

    hash as usize
}

/// Combine two hash values.
///
/// Matches Kakoune's `combine_hash` implementation in hash.hh.
#[unsafe(no_mangle)]
pub extern "C" fn combine_hash(lhs: usize, rhs: usize) -> usize {
    lhs ^ (rhs
        .wrapping_add(0x9e3779b9)
        .wrapping_add(lhs << 6)
        .wrapping_add(lhs >> 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur3_hello_world() {
        let data = b"Hello, World!";
        assert_eq!(murmur3(data.as_ptr(), data.len()), 0xf816f95b);
    }

    #[test]
    fn test_murmur3_xxxx() {
        let data = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        assert_eq!(murmur3(data.as_ptr(), data.len()), 3551113186);
    }

    #[test]
    fn test_murmur3_empty() {
        assert_eq!(murmur3(b"".as_ptr(), 0), 2572747774);
    }

    #[test]
    fn test_fnv1a_basic() {
        let data = b"test";
        let hash = fnv1a(data.as_ptr(), data.len());
        assert!(hash != 0);
    }

    #[test]
    fn test_combine_hash() {
        let h1 = 12345usize;
        let h2 = 67890usize;
        let combined = combine_hash(h1, h2);
        assert!(combined != h1 && combined != h2);
    }
}
