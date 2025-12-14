// Integration test: C++ calling Rust FFI hash, UTF-8, and Unicode functions
// This mimics how kakoune C++ code would call the Rust implementations

#include <cstdio>
#include <cstdint>
#include <cstring>
#include <cassert>

// Include the generated Rust FFI header
#include "kak_ffi.h"

// Test vectors from kakoune's hash.cc unit tests
void test_murmur3() {
    // Test 1: "Hello, World!" -> 0xf816f95b
    {
        const char* data = "Hello, World!";
        uintptr_t hash = murmur3(reinterpret_cast<const uint8_t*>(data), strlen(data));
        printf("murmur3(\"Hello, World!\") = 0x%lx (expected 0xf816f95b)\n", hash);
        assert(hash == 0xf816f95b);
    }

    // Test 2: 28 x's -> 3551113186
    {
        const char* data = "xxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        uintptr_t hash = murmur3(reinterpret_cast<const uint8_t*>(data), 28);
        printf("murmur3(\"xxxx...\" x28) = %lu (expected 3551113186)\n", hash);
        assert(hash == 3551113186);
    }

    // Test 3: empty string -> 2572747774
    {
        uintptr_t hash = murmur3(reinterpret_cast<const uint8_t*>(""), 0);
        printf("murmur3(\"\") = %lu (expected 2572747774)\n", hash);
        assert(hash == 2572747774);
    }

    printf("murmur3: all tests passed!\n\n");
}

void test_fnv1a() {
    const char* data = "test";
    uintptr_t hash = fnv1a(reinterpret_cast<const uint8_t*>(data), strlen(data));
    printf("fnv1a(\"test\") = 0x%lx\n", hash);
    assert(hash != 0);
    printf("fnv1a: basic test passed!\n\n");
}

void test_combine_hash() {
    uintptr_t h1 = 12345;
    uintptr_t h2 = 67890;
    uintptr_t combined = combine_hash(h1, h2);
    printf("combine_hash(%lu, %lu) = %lu\n", h1, h2, combined);
    assert(combined != h1 && combined != h2);
    printf("combine_hash: basic test passed!\n\n");
}

void test_utf8() {
    printf("--- UTF-8 Tests ---\n");

    // is_character_start
    assert(utf8_is_character_start('a'));
    assert(utf8_is_character_start(0xC2)); // 2-byte start
    assert(!utf8_is_character_start(0x80)); // continuation
    printf("utf8_is_character_start: passed\n");

    // codepoint_size_from_byte
    assert(utf8_codepoint_size_from_byte('a') == 1);
    assert(utf8_codepoint_size_from_byte(0xC2) == 2);
    assert(utf8_codepoint_size_from_byte(0xE0) == 3);
    assert(utf8_codepoint_size_from_byte(0xF0) == 4);
    printf("utf8_codepoint_size_from_byte: passed\n");

    // codepoint_size_from_codepoint
    assert(utf8_codepoint_size_from_codepoint(0x41) == 1);      // A
    assert(utf8_codepoint_size_from_codepoint(0xE9) == 2);      // é
    assert(utf8_codepoint_size_from_codepoint(0x20AC) == 3);    // €
    assert(utf8_codepoint_size_from_codepoint(0x1D11E) == 4);   // 𝄞
    printf("utf8_codepoint_size_from_codepoint: passed\n");

    // read_codepoint - ASCII
    {
        const uint8_t data[] = "Hello";
        uintptr_t offset = 0;
        assert(utf8_read_codepoint(data, 5, &offset) == 'H');
        assert(offset == 1);
        assert(utf8_read_codepoint(data, 5, &offset) == 'e');
        assert(offset == 2);
    }
    printf("utf8_read_codepoint (ASCII): passed\n");

    // read_codepoint - multibyte: é = U+00E9 = 0xC3 0xA9
    {
        const uint8_t data[] = {0xC3, 0xA9};
        uintptr_t offset = 0;
        assert(utf8_read_codepoint(data, 2, &offset) == 0xE9);
        assert(offset == 2);
    }
    printf("utf8_read_codepoint (2-byte): passed\n");

    // read_codepoint - 3-byte: € = U+20AC = 0xE2 0x82 0xAC
    {
        const uint8_t data[] = {0xE2, 0x82, 0xAC};
        uintptr_t offset = 0;
        assert(utf8_read_codepoint(data, 3, &offset) == 0x20AC);
        assert(offset == 3);
    }
    printf("utf8_read_codepoint (3-byte): passed\n");

    // read_codepoint - 4-byte: 𝄞 = U+1D11E = 0xF0 0x9D 0x84 0x9E
    {
        const uint8_t data[] = {0xF0, 0x9D, 0x84, 0x9E};
        uintptr_t offset = 0;
        assert(utf8_read_codepoint(data, 4, &offset) == 0x1D11E);
        assert(offset == 4);
    }
    printf("utf8_read_codepoint (4-byte): passed\n");

    // encode_codepoint
    {
        uint8_t buf[4];

        // ASCII
        assert(utf8_encode_codepoint('A', buf) == 1);
        assert(buf[0] == 'A');

        // 2-byte: é
        assert(utf8_encode_codepoint(0xE9, buf) == 2);
        assert(buf[0] == 0xC3 && buf[1] == 0xA9);

        // 3-byte: €
        assert(utf8_encode_codepoint(0x20AC, buf) == 3);
        assert(buf[0] == 0xE2 && buf[1] == 0x82 && buf[2] == 0xAC);

        // 4-byte: 𝄞
        assert(utf8_encode_codepoint(0x1D11E, buf) == 4);
        assert(buf[0] == 0xF0 && buf[1] == 0x9D && buf[2] == 0x84 && buf[3] == 0x9E);
    }
    printf("utf8_encode_codepoint: passed\n");

    // char_count
    {
        const char* ascii = "Hello";
        assert(utf8_char_count(reinterpret_cast<const uint8_t*>(ascii), 5) == 5);

        // "Héllo" - 5 characters, 6 bytes
        const uint8_t utf8_str[] = {'H', 0xC3, 0xA9, 'l', 'l', 'o'};
        assert(utf8_char_count(utf8_str, 6) == 5);
    }
    printf("utf8_char_count: passed\n");

    printf("UTF-8: all tests passed!\n\n");
}

void test_unicode() {
    printf("--- Unicode Tests ---\n");

    // is_eol
    assert(unicode_is_eol('\n'));
    assert(!unicode_is_eol('\r'));
    printf("unicode_is_eol: passed\n");

    // is_horizontal_blank
    assert(unicode_is_horizontal_blank('\t'));
    assert(unicode_is_horizontal_blank(' '));
    assert(unicode_is_horizontal_blank(0x00A0)); // NBSP
    assert(!unicode_is_horizontal_blank('\n'));
    printf("unicode_is_horizontal_blank: passed\n");

    // is_blank
    assert(unicode_is_blank('\n'));
    assert(unicode_is_blank('\r'));
    assert(unicode_is_blank('\t'));
    assert(unicode_is_blank(' '));
    assert(!unicode_is_blank('a'));
    printf("unicode_is_blank: passed\n");

    // is_basic_alpha
    assert(unicode_is_basic_alpha('a'));
    assert(unicode_is_basic_alpha('Z'));
    assert(!unicode_is_basic_alpha('5'));
    printf("unicode_is_basic_alpha: passed\n");

    // is_basic_digit
    assert(unicode_is_basic_digit('0'));
    assert(unicode_is_basic_digit('9'));
    assert(!unicode_is_basic_digit('a'));
    printf("unicode_is_basic_digit: passed\n");

    // is_word
    assert(unicode_is_word('a'));
    assert(unicode_is_word('5'));
    assert(unicode_is_word('_'));
    assert(!unicode_is_word('-'));
    assert(!unicode_is_word('.'));
    printf("unicode_is_word: passed\n");

    // is_word_big (WORD)
    assert(unicode_is_word_big('a'));
    assert(unicode_is_word_big('.'));
    assert(unicode_is_word_big('-'));
    assert(!unicode_is_word_big(' '));
    assert(!unicode_is_word_big('\n'));
    printf("unicode_is_word_big: passed\n");

    // is_punctuation
    assert(unicode_is_punctuation('.'));
    assert(unicode_is_punctuation('-'));
    assert(!unicode_is_punctuation('a'));
    assert(!unicode_is_punctuation(' '));
    printf("unicode_is_punctuation: passed\n");

    // is_identifier
    assert(unicode_is_identifier('a'));
    assert(unicode_is_identifier('5'));
    assert(unicode_is_identifier('_'));
    assert(unicode_is_identifier('-'));
    assert(!unicode_is_identifier('.'));
    printf("unicode_is_identifier: passed\n");

    // case conversion
    assert(unicode_to_lower_ascii('A') == 'a');
    assert(unicode_to_upper_ascii('a') == 'A');
    assert(unicode_to_lower('A') == 'a');
    assert(unicode_to_upper('a') == 'A');
    printf("unicode case conversion: passed\n");

    // is_lower/is_upper
    assert(unicode_is_lower_ascii('a'));
    assert(!unicode_is_lower_ascii('A'));
    assert(unicode_is_upper_ascii('A'));
    assert(!unicode_is_upper_ascii('a'));
    assert(unicode_is_lower('a'));
    assert(unicode_is_upper('A'));
    printf("unicode is_lower/is_upper: passed\n");

    // codepoint_width
    assert(unicode_codepoint_width('a') == 1);
    assert(unicode_codepoint_width('\n') == 1);
    printf("unicode_codepoint_width: passed\n");

    // categorize
    assert(unicode_categorize('\n') == EndOfLine);
    assert(unicode_categorize(' ') == Blank);
    assert(unicode_categorize('a') == Word);
    assert(unicode_categorize('.') == Punctuation);
    printf("unicode_categorize: passed\n");

    // categorize_word (WORD mode)
    assert(unicode_categorize_word('\n') == EndOfLine);
    assert(unicode_categorize_word(' ') == Blank);
    assert(unicode_categorize_word('.') == Word); // WORD mode: punctuation is Word
    printf("unicode_categorize_word: passed\n");

    printf("Unicode: all tests passed!\n\n");
}

int main() {
    printf("=== kak-ffi C++ Integration Test ===\n\n");

    test_murmur3();
    test_fnv1a();
    test_combine_hash();
    test_utf8();
    test_unicode();

    printf("=== All integration tests passed! ===\n");
    return 0;
}
