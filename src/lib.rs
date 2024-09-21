//! Simple, reusable and optimized XOR ciphers in Rust.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Applies XOR operation (`byte ^ key`) for each `byte` in `data`.
///
/// This function is its own inverse.
#[inline]
pub fn xor<D: AsMut<[u8]>>(mut data: D, key: u8) {
    fn xor_inner(data: &mut [u8], key: u8) {
        data.iter_mut().for_each(|byte| *byte ^= key);
    }

    xor_inner(data.as_mut(), key);
}

/// Applies XOR operation (`byte ^ key_byte`) for each `byte` in `data`
/// and `key_byte` in `key`, which is cycled to fit the length of the `data`.
///
/// This function is its own inverse.
#[inline]
pub fn cyclic_xor<D: AsMut<[u8]>, K: AsRef<[u8]>>(mut data: D, key: K) {
    fn cyclic_xor_inner(data: &mut [u8], key: &[u8]) {
        data.iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(byte, key_byte)| *byte ^= key_byte);
    }

    cyclic_xor_inner(data.as_mut(), key.as_ref());
}
