//! In-place XOR ciphers.

/// Applies XOR operation (`byte ^ key`) for each `byte` in `data`.
///
/// This function is its own inverse.
#[inline]
pub fn xor_in_place(data: &mut [u8], key: u8) {
    data.iter_mut().for_each(|byte| *byte ^= key);
}

/// Applies XOR operation (`byte ^ key_byte`) for each `byte` in `data`
/// and `key_byte` in `key`, which is cycled to fit the length of the `data`.
///
/// This function is its own inverse.
#[inline]
pub fn cyclic_xor_in_place(data: &mut [u8], key: &[u8]) {
    data.iter_mut()
        .zip(key.iter().cycle())
        .for_each(|(byte, key_byte)| *byte ^= key_byte);
}
