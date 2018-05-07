//! `HMAC-SHA-512` `HMAC-SHA-512` is conjectured to meet the standard notion of
//! unforgeability.

use ffi::{crypto_auth_hmacsha512, crypto_auth_hmacsha512_BYTES, crypto_auth_hmacsha512_KEYBYTES,
          crypto_auth_hmacsha512_final, crypto_auth_hmacsha512_init, crypto_auth_hmacsha512_state,
          crypto_auth_hmacsha512_update, crypto_auth_hmacsha512_verify};

auth_module!(
    crypto_auth_hmacsha512,
    crypto_auth_hmacsha512_verify,
    crypto_auth_hmacsha512_KEYBYTES as usize,
    crypto_auth_hmacsha512_BYTES as usize
);

auth_state!(
    crypto_auth_hmacsha512_state,
    crypto_auth_hmacsha512_init,
    crypto_auth_hmacsha512_update,
    crypto_auth_hmacsha512_final,
    crypto_auth_hmacsha512_BYTES
);

#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod test {
    use super::*;

    #[test]
    fn test_vector_1() {
        // corresponding to tests/auth.c from NaCl
        // "Test Case 2" from RFC 4231
        unwrap!(::init());
        let key = Key([0x4a, 0x65, 0x66, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let c = [0x77, 0x68, 0x61, 0x74, 0x20, 0x64, 0x6f, 0x20, 0x79, 0x61, 0x20, 0x77, 0x61,
                 0x6e, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x68, 0x69, 0x6e,
                 0x67, 0x3f];

        let a_expected = [0x16, 0x4b, 0x7a, 0x7b, 0xfc, 0xf8, 0x19, 0xe2, 0xe3, 0x95, 0xfb, 0xe7,
                          0x3b, 0x56, 0xe0, 0xa3, 0x87, 0xbd, 0x64, 0x22, 0x2e, 0x83, 0x1f, 0xd6,
                          0x10, 0x27, 0x0c, 0xd7, 0xea, 0x25, 0x05, 0x54, 0x97, 0x58, 0xbf, 0x75,
                          0xc0, 0x5a, 0x99, 0x4a, 0x6d, 0x03, 0x4f, 0x65, 0xf8, 0xf0, 0xe6, 0xfd,
                          0xca, 0xea, 0xb1, 0xa3, 0x4d, 0x4a, 0x6b, 0x4b, 0x63, 0x6e, 0x07, 0x0a,
                          0x38, 0xbc, 0xe7, 0x37];

        let Tag(a) = authenticate(&c, &key);
        assert!(a[..] == a_expected[..]);
    }

    #[test]
    fn test_vector_state_1() {
        // corresponding to tests/auth.c from NaCl
        // "Test Case 2" from RFC 4231
        unwrap!(::init());
        let key = [0x4a, 0x65, 0x66, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let c = [0x77, 0x68, 0x61, 0x74, 0x20, 0x64, 0x6f, 0x20, 0x79, 0x61, 0x20, 0x77, 0x61,
                 0x6e, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x68, 0x69, 0x6e,
                 0x67, 0x3f];

        let a_expected = [0x16, 0x4b, 0x7a, 0x7b, 0xfc, 0xf8, 0x19, 0xe2, 0xe3, 0x95, 0xfb, 0xe7,
                          0x3b, 0x56, 0xe0, 0xa3, 0x87, 0xbd, 0x64, 0x22, 0x2e, 0x83, 0x1f, 0xd6,
                          0x10, 0x27, 0x0c, 0xd7, 0xea, 0x25, 0x05, 0x54, 0x97, 0x58, 0xbf, 0x75,
                          0xc0, 0x5a, 0x99, 0x4a, 0x6d, 0x03, 0x4f, 0x65, 0xf8, 0xf0, 0xe6, 0xfd,
                          0xca, 0xea, 0xb1, 0xa3, 0x4d, 0x4a, 0x6b, 0x4b, 0x63, 0x6e, 0x07, 0x0a,
                          0x38, 0xbc, 0xe7, 0x37];

        let mut state = State::init(&key);
        state.update(&c);
        let Tag(a) = state.finalize();
        assert!(a[..] == a_expected[..]);
    }
}
