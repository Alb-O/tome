//! Unicode character classification functions matching Kakoune's unicode.hh

// Wide character functions from libc not exposed in the libc crate
unsafe extern "C" {
    fn iswalnum(wc: libc::wchar_t) -> libc::c_int;
    fn iswlower(wc: libc::wchar_t) -> libc::c_int;
    fn iswupper(wc: libc::wchar_t) -> libc::c_int;
    fn towlower(wc: libc::wchar_t) -> libc::wchar_t;
    fn towupper(wc: libc::wchar_t) -> libc::wchar_t;
    fn wcwidth(wc: libc::wchar_t) -> libc::c_int;
}

/// Check if codepoint is end-of-line (newline).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_eol(c: u32) -> bool {
    c == '\n' as u32
}

/// Check if codepoint is horizontal blank (whitespace excluding vertical).
///
/// Characters considered whitespace by ECMA Regex Spec minus vertical tab.
/// <https://262.ecma-international.org/11.0/#sec-white-space>
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_horizontal_blank(c: u32) -> bool {
    matches!(
        c,
        0x0009     // \t - tab
        | 0x000C   // \f - form feed
        | 0x0020   // space
        | 0x00A0   // no-break space
        | 0xFEFF   // zero-width no-break space (BOM)
        | 0x1680   // ogham space mark
        | 0x2000   // en quad
        | 0x2001   // em quad
        | 0x2002   // en space
        | 0x2003   // em space
        | 0x2004   // three-per-em space
        | 0x2005   // four-per-em space
        | 0x2006   // six-per-em space
        | 0x2007   // figure space
        | 0x2008   // punctuation space
        | 0x2009   // thin space
        | 0x200A   // hair space
        | 0x2028   // line separator
        | 0x2029   // paragraph separator
        | 0x202F   // narrow no-break space
        | 0x205F   // medium mathematical space
        | 0x3000   // ideographic space
    )
}

/// Check if codepoint is blank (whitespace including line terminators).
///
/// Characters considered Line Terminators by ECMA Regex Spec plus vertical tab.
/// <https://262.ecma-international.org/11.0/#sec-line-terminators>
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_blank(c: u32) -> bool {
    matches!(
        c,
        0x000A     // \n - newline
        | 0x000D   // \r - carriage return
        | 0x000B   // \v - vertical tab
        | 0x2028   // line separator
        | 0x2029   // paragraph separator
    ) || unicode_is_horizontal_blank(c)
}

/// Check if codepoint is a basic ASCII letter (a-z, A-Z).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_basic_alpha(c: u32) -> bool {
    (c >= 'a' as u32 && c <= 'z' as u32) || (c >= 'A' as u32 && c <= 'Z' as u32)
}

/// Check if codepoint is a basic ASCII digit (0-9).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_basic_digit(c: u32) -> bool {
    c >= '0' as u32 && c <= '9' as u32
}

/// Check if codepoint is alphanumeric (ASCII fast path, Unicode fallback).
///
/// For non-ASCII, uses libc iswalnum.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_alnum(c: u32) -> bool {
    if c < 128 {
        unicode_is_basic_alpha(c) || unicode_is_basic_digit(c)
    } else {
        unsafe { iswalnum(c as libc::wchar_t) != 0 }
    }
}

/// Check if codepoint is a word character.
///
/// Word characters are alphanumeric or underscore.
/// For non-ASCII, defers to libc iswalnum.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_word(c: u32) -> bool {
    c == '_' as u32 || unicode_is_alnum(c)
}

/// Check if codepoint is a WORD character (non-blank).
///
/// In kakoune, WORD means any non-blank character.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_word_big(c: u32) -> bool {
    !unicode_is_blank(c)
}

/// Check if codepoint is punctuation (not word and not blank).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_punctuation(c: u32) -> bool {
    !unicode_is_word(c) && !unicode_is_blank(c)
}

/// Check if codepoint is an identifier character.
///
/// Identifiers consist of: a-z, A-Z, 0-9, underscore, hyphen.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_identifier(c: u32) -> bool {
    unicode_is_basic_alpha(c) || unicode_is_basic_digit(c) || c == '_' as u32 || c == '-' as u32
}

/// Convert ASCII character to lowercase.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_to_lower_ascii(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        c - b'A' + b'a'
    } else {
        c
    }
}

/// Convert ASCII character to uppercase.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_to_upper_ascii(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        c - b'a' + b'A'
    } else {
        c
    }
}

/// Check if ASCII character is lowercase.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_lower_ascii(c: u8) -> bool {
    c >= b'a' && c <= b'z'
}

/// Check if ASCII character is uppercase.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_upper_ascii(c: u8) -> bool {
    c >= b'A' && c <= b'Z'
}

/// Convert codepoint to lowercase.
///
/// Uses ASCII fast path, then libc towlower for non-ASCII.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_to_lower(cp: u32) -> u32 {
    if cp < 128 {
        unicode_to_lower_ascii(cp as u8) as u32
    } else {
        unsafe { towlower(cp as libc::wchar_t) as u32 }
    }
}

/// Convert codepoint to uppercase.
///
/// Uses ASCII fast path, then libc towupper for non-ASCII.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_to_upper(cp: u32) -> u32 {
    if cp < 128 {
        unicode_to_upper_ascii(cp as u8) as u32
    } else {
        unsafe { towupper(cp as libc::wchar_t) as u32 }
    }
}

/// Check if codepoint is lowercase.
///
/// Uses ASCII fast path, then libc iswlower for non-ASCII.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_lower(cp: u32) -> bool {
    if cp < 128 {
        unicode_is_lower_ascii(cp as u8)
    } else {
        unsafe { iswlower(cp as libc::wchar_t) != 0 }
    }
}

/// Check if codepoint is uppercase.
///
/// Uses ASCII fast path, then libc iswupper for non-ASCII.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_is_upper(cp: u32) -> bool {
    if cp < 128 {
        unicode_is_upper_ascii(cp as u8)
    } else {
        unsafe { iswupper(cp as libc::wchar_t) != 0 }
    }
}

/// Get display width of a codepoint.
///
/// Returns 1 for newline, otherwise uses libc wcwidth.
/// Returns 1 for control characters (wcwidth returns -1).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_codepoint_width(c: u32) -> i32 {
    if c == '\n' as u32 {
        return 1;
    }
    let width = unsafe { wcwidth(c as libc::wchar_t) };
    if width >= 0 { width } else { 1 }
}

/// Character category for word motion.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharCategory {
    Blank = 0,
    EndOfLine = 1,
    Word = 2,
    Punctuation = 3,
}

/// Categorize a codepoint for word motion.
#[unsafe(no_mangle)]
pub extern "C" fn unicode_categorize(c: u32) -> CharCategory {
    if unicode_is_eol(c) {
        CharCategory::EndOfLine
    } else if unicode_is_horizontal_blank(c) {
        CharCategory::Blank
    } else if unicode_is_word(c) {
        CharCategory::Word
    } else {
        CharCategory::Punctuation
    }
}

/// Categorize a codepoint for WORD motion (any non-blank is Word).
#[unsafe(no_mangle)]
pub extern "C" fn unicode_categorize_word(c: u32) -> CharCategory {
    if unicode_is_eol(c) {
        CharCategory::EndOfLine
    } else if unicode_is_horizontal_blank(c) {
        CharCategory::Blank
    } else {
        CharCategory::Word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_eol() {
        assert!(unicode_is_eol('\n' as u32));
        assert!(!unicode_is_eol('\r' as u32));
        assert!(!unicode_is_eol(' ' as u32));
    }

    #[test]
    fn test_is_horizontal_blank() {
        assert!(unicode_is_horizontal_blank('\t' as u32));
        assert!(unicode_is_horizontal_blank(' ' as u32));
        assert!(unicode_is_horizontal_blank(0x00A0)); // NBSP
        assert!(unicode_is_horizontal_blank(0x3000)); // ideographic space
        assert!(!unicode_is_horizontal_blank('\n' as u32));
        assert!(!unicode_is_horizontal_blank('a' as u32));
    }

    #[test]
    fn test_is_blank() {
        assert!(unicode_is_blank('\n' as u32));
        assert!(unicode_is_blank('\r' as u32));
        assert!(unicode_is_blank('\t' as u32));
        assert!(unicode_is_blank(' ' as u32));
        assert!(unicode_is_blank(0x2028)); // line separator
        assert!(!unicode_is_blank('a' as u32));
    }

    #[test]
    fn test_is_basic_alpha() {
        assert!(unicode_is_basic_alpha('a' as u32));
        assert!(unicode_is_basic_alpha('z' as u32));
        assert!(unicode_is_basic_alpha('A' as u32));
        assert!(unicode_is_basic_alpha('Z' as u32));
        assert!(!unicode_is_basic_alpha('0' as u32));
        assert!(!unicode_is_basic_alpha(' ' as u32));
        assert!(!unicode_is_basic_alpha(0xE9)); // é
    }

    #[test]
    fn test_is_basic_digit() {
        assert!(unicode_is_basic_digit('0' as u32));
        assert!(unicode_is_basic_digit('9' as u32));
        assert!(!unicode_is_basic_digit('a' as u32));
    }

    #[test]
    fn test_is_word() {
        assert!(unicode_is_word('a' as u32));
        assert!(unicode_is_word('Z' as u32));
        assert!(unicode_is_word('5' as u32));
        assert!(unicode_is_word('_' as u32));
        assert!(!unicode_is_word('-' as u32));
        assert!(!unicode_is_word(' ' as u32));
        assert!(!unicode_is_word('.' as u32));
    }

    #[test]
    fn test_is_punctuation() {
        assert!(unicode_is_punctuation('.' as u32));
        assert!(unicode_is_punctuation(',' as u32));
        assert!(unicode_is_punctuation('!' as u32));
        assert!(unicode_is_punctuation('-' as u32));
        assert!(!unicode_is_punctuation('a' as u32));
        assert!(!unicode_is_punctuation(' ' as u32));
        assert!(!unicode_is_punctuation('_' as u32));
    }

    #[test]
    fn test_is_identifier() {
        assert!(unicode_is_identifier('a' as u32));
        assert!(unicode_is_identifier('Z' as u32));
        assert!(unicode_is_identifier('0' as u32));
        assert!(unicode_is_identifier('_' as u32));
        assert!(unicode_is_identifier('-' as u32));
        assert!(!unicode_is_identifier(' ' as u32));
        assert!(!unicode_is_identifier('.' as u32));
    }

    #[test]
    fn test_case_conversion_ascii() {
        assert_eq!(unicode_to_lower_ascii(b'A'), b'a');
        assert_eq!(unicode_to_lower_ascii(b'Z'), b'z');
        assert_eq!(unicode_to_lower_ascii(b'a'), b'a');
        assert_eq!(unicode_to_lower_ascii(b'5'), b'5');

        assert_eq!(unicode_to_upper_ascii(b'a'), b'A');
        assert_eq!(unicode_to_upper_ascii(b'z'), b'Z');
        assert_eq!(unicode_to_upper_ascii(b'A'), b'A');
    }

    #[test]
    fn test_is_lower_upper_ascii() {
        assert!(unicode_is_lower_ascii(b'a'));
        assert!(unicode_is_lower_ascii(b'z'));
        assert!(!unicode_is_lower_ascii(b'A'));
        assert!(!unicode_is_lower_ascii(b'5'));

        assert!(unicode_is_upper_ascii(b'A'));
        assert!(unicode_is_upper_ascii(b'Z'));
        assert!(!unicode_is_upper_ascii(b'a'));
    }

    #[test]
    fn test_case_conversion_codepoint() {
        assert_eq!(unicode_to_lower('A' as u32), 'a' as u32);
        assert_eq!(unicode_to_upper('a' as u32), 'A' as u32);
    }

    #[test]
    fn test_codepoint_width() {
        assert_eq!(unicode_codepoint_width('a' as u32), 1);
        assert_eq!(unicode_codepoint_width('\n' as u32), 1);
        // CJK width depends on locale; just verify it returns something reasonable
        let cjk_width = unicode_codepoint_width(0x4E2D); // 中
        assert!(cjk_width == 1 || cjk_width == 2);
    }

    #[test]
    fn test_categorize() {
        assert_eq!(unicode_categorize('\n' as u32), CharCategory::EndOfLine);
        assert_eq!(unicode_categorize(' ' as u32), CharCategory::Blank);
        assert_eq!(unicode_categorize('\t' as u32), CharCategory::Blank);
        assert_eq!(unicode_categorize('a' as u32), CharCategory::Word);
        assert_eq!(unicode_categorize('5' as u32), CharCategory::Word);
        assert_eq!(unicode_categorize('_' as u32), CharCategory::Word);
        assert_eq!(unicode_categorize('.' as u32), CharCategory::Punctuation);
        assert_eq!(unicode_categorize('-' as u32), CharCategory::Punctuation);
    }

    #[test]
    fn test_categorize_word() {
        assert_eq!(unicode_categorize_word('\n' as u32), CharCategory::EndOfLine);
        assert_eq!(unicode_categorize_word(' ' as u32), CharCategory::Blank);
        // WORD mode: everything non-blank is Word
        assert_eq!(unicode_categorize_word('.' as u32), CharCategory::Word);
        assert_eq!(unicode_categorize_word('-' as u32), CharCategory::Word);
    }
}
