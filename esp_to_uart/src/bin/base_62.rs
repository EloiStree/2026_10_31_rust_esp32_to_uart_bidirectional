
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// 65-character alphabet: digits, upper, lower, then '-', '.', '_'.
pub const BASE65_ALPHABET: &[u8; 65] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-._";

/// Convert bytes to a Base65 string. Leading zero bytes are kept as leading '0' chars.
pub fn bytes_to_base65(value: &[u8]) -> String {
    let leading_zeros = value.iter().take_while(|&&b| b == 0).count();
    // Base65 digits, least significant first.
    let mut digits: Vec<u8> = Vec::new();
    for &byte in &value[leading_zeros..] {
        let mut carry = byte as u32;
        for d in digits.iter_mut() {
            carry += (*d as u32) << 8;
            *d = (carry % 65) as u8;
            carry /= 65;
        }
        while carry > 0 {
            digits.push((carry % 65) as u8);
            carry /= 65;
        }
    }
    let mut result = String::with_capacity(leading_zeros + digits.len());
    for _ in 0..leading_zeros {
        result.push(BASE65_ALPHABET[0] as char);
    }
    for &d in digits.iter().rev() {
        result.push(BASE65_ALPHABET[d as usize] as char);
    }
    result
}

/// Convert a Base65 string to bytes.
pub fn base65_to_bytes(value: &str) -> Result<Vec<u8>, String> {
    if value.is_empty() {
        return Err(String::from("Base65 value cannot be empty"));
    }
    let leading_zeros = value.bytes().take_while(|&c| c == BASE65_ALPHABET[0]).count();
    // Bytes, least significant first.
    let mut bytes: Vec<u8> = Vec::new();
    for c in value.bytes().skip(leading_zeros) {
        let digit = BASE65_ALPHABET
            .iter()
            .position(|&a| a == c)
            .ok_or_else(|| {
                let mut msg = String::from("Invalid Base65 character: ");
                msg.push(c as char);
                msg
            })?;
        let mut carry = digit as u32;
        for b in bytes.iter_mut() {
            carry += (*b as u32) * 65;
            *b = (carry & 0xFF) as u8;
            carry >>= 8;
        }
        while carry > 0 {
            bytes.push((carry & 0xFF) as u8);
            carry >>= 8;
        }
    }
    let mut result = alloc::vec![0u8; leading_zeros];
    result.extend(bytes.iter().rev());
    Ok(result)
}
