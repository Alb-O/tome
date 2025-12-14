//! UTF-8 encoding and decoding functions matching Kakoune's utf8.hh

/// Check if byte is the start of a UTF-8 character (single or multibyte).
///
/// Returns true if the byte is not a continuation byte (0b10xxxxxx).
#[unsafe(no_mangle)]
pub extern "C" fn utf8_is_character_start(byte: u8) -> bool {
    (byte & 0xC0) != 0x80
}

/// Get the number of bytes needed to encode a codepoint in UTF-8.
///
/// Returns 0 for invalid codepoints (> 0x10FFFF).
#[unsafe(no_mangle)]
pub extern "C" fn utf8_codepoint_size_from_codepoint(cp: u32) -> usize {
    if cp <= 0x7F {
        1
    } else if cp <= 0x7FF {
        2
    } else if cp <= 0xFFFF {
        3
    } else if cp <= 0x10FFFF {
        4
    } else {
        0
    }
}

/// Get the number of bytes in a UTF-8 character from its first byte.
///
/// Returns 1 for invalid/continuation bytes.
#[unsafe(no_mangle)]
pub extern "C" fn utf8_codepoint_size_from_byte(byte: u8) -> usize {
    if (byte & 0x80) == 0 {
        1 // 0xxxxxxx - ASCII
    } else if (byte & 0xE0) == 0xC0 {
        2 // 110xxxxx
    } else if (byte & 0xF0) == 0xE0 {
        3 // 1110xxxx
    } else if (byte & 0xF8) == 0xF0 {
        4 // 11110xxx
    } else {
        1 // Invalid or continuation byte
    }
}

/// Decode a UTF-8 sequence to a codepoint.
///
/// Returns the codepoint and advances `*offset` by the number of bytes consumed.
/// Returns 0xFFFD (replacement char) for invalid sequences.
/// If offset >= len, returns 0xFFFD and does not advance offset.
#[unsafe(no_mangle)]
pub extern "C" fn utf8_read_codepoint(data: *const u8, len: usize, offset: *mut usize) -> u32 {
    const REPLACEMENT: u32 = 0xFFFD;

    if data.is_null() || offset.is_null() {
        return REPLACEMENT;
    }

    let off = unsafe { *offset };
    if off >= len {
        return REPLACEMENT;
    }

    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    let byte0 = bytes[off];

    // ASCII fast path
    if (byte0 & 0x80) == 0 {
        unsafe { *offset = off + 1 };
        return byte0 as u32;
    }

    // 2-byte sequence: 110xxxxx 10xxxxxx
    if (byte0 & 0xE0) == 0xC0 {
        if off + 1 >= len {
            unsafe { *offset = off + 1 };
            return REPLACEMENT;
        }
        let byte1 = bytes[off + 1];
        unsafe { *offset = off + 2 };
        return ((byte0 as u32 & 0x1F) << 6) | (byte1 as u32 & 0x3F);
    }

    // 3-byte sequence: 1110xxxx 10xxxxxx 10xxxxxx
    if (byte0 & 0xF0) == 0xE0 {
        if off + 2 >= len {
            unsafe { *offset = off + 1 };
            return REPLACEMENT;
        }
        let byte1 = bytes[off + 1];
        let byte2 = bytes[off + 2];
        unsafe { *offset = off + 3 };
        return ((byte0 as u32 & 0x0F) << 12)
            | ((byte1 as u32 & 0x3F) << 6)
            | (byte2 as u32 & 0x3F);
    }

    // 4-byte sequence: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
    if (byte0 & 0xF8) == 0xF0 {
        if off + 3 >= len {
            unsafe { *offset = off + 1 };
            return REPLACEMENT;
        }
        let byte1 = bytes[off + 1];
        let byte2 = bytes[off + 2];
        let byte3 = bytes[off + 3];
        unsafe { *offset = off + 4 };
        return ((byte0 as u32 & 0x07) << 18)
            | ((byte1 as u32 & 0x3F) << 12)
            | ((byte2 as u32 & 0x3F) << 6)
            | (byte3 as u32 & 0x3F);
    }

    // Invalid leading byte
    unsafe { *offset = off + 1 };
    REPLACEMENT
}

/// Encode a codepoint to UTF-8.
///
/// Writes to `out` buffer (must have space for at least 4 bytes).
/// Returns the number of bytes written, or 0 for invalid codepoints.
#[unsafe(no_mangle)]
pub extern "C" fn utf8_encode_codepoint(cp: u32, out: *mut u8) -> usize {
    if out.is_null() {
        return 0;
    }

    let out_slice = unsafe { std::slice::from_raw_parts_mut(out, 4) };

    if cp <= 0x7F {
        out_slice[0] = cp as u8;
        1
    } else if cp <= 0x7FF {
        out_slice[0] = 0xC0 | ((cp >> 6) as u8);
        out_slice[1] = 0x80 | ((cp & 0x3F) as u8);
        2
    } else if cp <= 0xFFFF {
        out_slice[0] = 0xE0 | ((cp >> 12) as u8);
        out_slice[1] = 0x80 | (((cp >> 6) & 0x3F) as u8);
        out_slice[2] = 0x80 | ((cp & 0x3F) as u8);
        3
    } else if cp <= 0x10FFFF {
        out_slice[0] = 0xF0 | ((cp >> 18) as u8);
        out_slice[1] = 0x80 | (((cp >> 12) & 0x3F) as u8);
        out_slice[2] = 0x80 | (((cp >> 6) & 0x3F) as u8);
        out_slice[3] = 0x80 | ((cp & 0x3F) as u8);
        4
    } else {
        0
    }
}

/// Count the number of UTF-8 characters in a byte slice.
///
/// This counts character start bytes only.
#[unsafe(no_mangle)]
pub extern "C" fn utf8_char_count(data: *const u8, len: usize) -> usize {
    if data.is_null() || len == 0 {
        return 0;
    }

    let bytes = unsafe { std::slice::from_raw_parts(data, len) };
    bytes.iter().filter(|&&b| utf8_is_character_start(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_character_start() {
        // ASCII
        assert!(utf8_is_character_start(b'a'));
        assert!(utf8_is_character_start(b'Z'));
        assert!(utf8_is_character_start(0x00));
        assert!(utf8_is_character_start(0x7F));

        // Multi-byte start bytes
        assert!(utf8_is_character_start(0xC2)); // 2-byte start
        assert!(utf8_is_character_start(0xE0)); // 3-byte start
        assert!(utf8_is_character_start(0xF0)); // 4-byte start

        // Continuation bytes should return false
        assert!(!utf8_is_character_start(0x80));
        assert!(!utf8_is_character_start(0xBF));
        assert!(!utf8_is_character_start(0xA0));
    }

    #[test]
    fn test_codepoint_size_from_codepoint() {
        assert_eq!(utf8_codepoint_size_from_codepoint(0x00), 1);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x7F), 1);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x80), 2);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x7FF), 2);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x800), 3);
        assert_eq!(utf8_codepoint_size_from_codepoint(0xFFFF), 3);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x10000), 4);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x10FFFF), 4);
        assert_eq!(utf8_codepoint_size_from_codepoint(0x110000), 0); // Invalid
    }

    #[test]
    fn test_codepoint_size_from_byte() {
        assert_eq!(utf8_codepoint_size_from_byte(b'a'), 1);
        assert_eq!(utf8_codepoint_size_from_byte(0x00), 1);
        assert_eq!(utf8_codepoint_size_from_byte(0x7F), 1);
        assert_eq!(utf8_codepoint_size_from_byte(0xC2), 2);
        assert_eq!(utf8_codepoint_size_from_byte(0xDF), 2);
        assert_eq!(utf8_codepoint_size_from_byte(0xE0), 3);
        assert_eq!(utf8_codepoint_size_from_byte(0xEF), 3);
        assert_eq!(utf8_codepoint_size_from_byte(0xF0), 4);
        assert_eq!(utf8_codepoint_size_from_byte(0xF4), 4);
        assert_eq!(utf8_codepoint_size_from_byte(0x80), 1); // Continuation - returns 1
        assert_eq!(utf8_codepoint_size_from_byte(0xBF), 1); // Continuation - returns 1
    }

    #[test]
    fn test_read_codepoint_ascii() {
        let data = b"Hello";
        let mut offset = 0usize;

        assert_eq!(utf8_read_codepoint(data.as_ptr(), data.len(), &mut offset), b'H' as u32);
        assert_eq!(offset, 1);

        assert_eq!(utf8_read_codepoint(data.as_ptr(), data.len(), &mut offset), b'e' as u32);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_read_codepoint_multibyte() {
        // "é" = U+00E9 = 0xC3 0xA9
        let data = [0xC3u8, 0xA9];
        let mut offset = 0usize;
        assert_eq!(utf8_read_codepoint(data.as_ptr(), data.len(), &mut offset), 0xE9);
        assert_eq!(offset, 2);

        // "€" = U+20AC = 0xE2 0x82 0xAC
        let data = [0xE2u8, 0x82, 0xAC];
        let mut offset = 0usize;
        assert_eq!(utf8_read_codepoint(data.as_ptr(), data.len(), &mut offset), 0x20AC);
        assert_eq!(offset, 3);

        // "𝄞" = U+1D11E = 0xF0 0x9D 0x84 0x9E
        let data = [0xF0u8, 0x9D, 0x84, 0x9E];
        let mut offset = 0usize;
        assert_eq!(utf8_read_codepoint(data.as_ptr(), data.len(), &mut offset), 0x1D11E);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_encode_codepoint() {
        let mut buf = [0u8; 4];

        // ASCII
        assert_eq!(utf8_encode_codepoint(b'A' as u32, buf.as_mut_ptr()), 1);
        assert_eq!(buf[0], b'A');

        // 2-byte: é = U+00E9
        assert_eq!(utf8_encode_codepoint(0xE9, buf.as_mut_ptr()), 2);
        assert_eq!(&buf[..2], &[0xC3, 0xA9]);

        // 3-byte: € = U+20AC
        assert_eq!(utf8_encode_codepoint(0x20AC, buf.as_mut_ptr()), 3);
        assert_eq!(&buf[..3], &[0xE2, 0x82, 0xAC]);

        // 4-byte: 𝄞 = U+1D11E
        assert_eq!(utf8_encode_codepoint(0x1D11E, buf.as_mut_ptr()), 4);
        assert_eq!(&buf[..4], &[0xF0, 0x9D, 0x84, 0x9E]);

        // Invalid
        assert_eq!(utf8_encode_codepoint(0x110000, buf.as_mut_ptr()), 0);
    }

    #[test]
    fn test_char_count() {
        // ASCII
        let data = b"Hello";
        assert_eq!(utf8_char_count(data.as_ptr(), data.len()), 5);

        // "Héllo" - 5 characters, 6 bytes
        let data = [b'H', 0xC3, 0xA9, b'l', b'l', b'o'];
        assert_eq!(utf8_char_count(data.as_ptr(), data.len()), 5);

        // Empty
        assert_eq!(utf8_char_count(b"".as_ptr(), 0), 0);
    }

    #[test]
    fn test_roundtrip() {
        let codepoints = [0x41u32, 0xE9, 0x20AC, 0x1D11E, 0x10FFFF];
        let mut buf = [0u8; 4];

        for &cp in &codepoints {
            let len = utf8_encode_codepoint(cp, buf.as_mut_ptr());
            assert!(len > 0);

            let mut offset = 0usize;
            let decoded = utf8_read_codepoint(buf.as_ptr(), len, &mut offset);
            assert_eq!(decoded, cp);
            assert_eq!(offset, len);
        }
    }
}
