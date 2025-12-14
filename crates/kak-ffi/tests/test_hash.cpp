// Integration test: C++ calling Rust FFI hash functions
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

int main() {
    printf("=== kak-ffi C++ Integration Test ===\n\n");

    test_murmur3();
    test_fnv1a();
    test_combine_hash();

    printf("=== All integration tests passed! ===\n");
    return 0;
}
