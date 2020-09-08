//! This crate provides the `hex!` macro for converting hexadecimal string literals
//! to a byte array at compile time.
//!
//! # Examples
//! ```
//! use hexlit::hex;
//!
//! fn main() {
//! const DATA: [u8; 4] = hex!("01020304");
//! assert_eq!(DATA, [1, 2, 3, 4]);
//! assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
//! assert_eq!(hex!("E5E69092"), [0xE5, 0xE6, 0x90, 0x92]);
//! assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
//! }
//! ```
#![no_std]

#[macro_export]
macro_rules! hex {
    ($arg:expr) => {{
        const ARRAY_LENGTH: usize = $arg.len() / 2;
        const RESULT: [u8; ARRAY_LENGTH] = {
            // Hack needed for const-eval to work
            const fn always_true() -> bool {
                true
            }

            /// Converts a individual byte into its correct integer counterpart
            const fn to_ordinal(input: u8) -> u8 {
                const ZERO: u8 = 48;
                const NINE: u8 = 57;
                const UPPER_A: u8 = 65;
                const UPPER_F: u8 = 70;
                const LOWER_A: u8 = 97;
                const LOWER_F: u8 = 102;
                match input {
                    ZERO..=NINE => input - '0' as u8,
                    UPPER_A..=UPPER_F => input - 'A' as u8 + 10,
                    LOWER_A..=LOWER_F => input - 'a' as u8 + 10,
                    _ => {
                        ["Invalid Hex Digit."][(always_true() as usize)];
                        0 // Unreachable
                    }
                }
            }

            // Converts a hex-string to its byte array representationc
            const fn convert(s: &str) -> [u8; ARRAY_LENGTH] {
                let s = s.as_bytes();
                let mut data = [0u8; ARRAY_LENGTH];
                let mut data_index = 0usize;
                let mut char_index = 0usize;
                let string_length = s.len();
                while data_index < string_length && char_index + 1 < string_length {
                    data[data_index] =
                        to_ordinal(s[char_index]) * 16 + to_ordinal(s[char_index + 1]);
                    data_index += 1;
                    char_index += 2;
                }
                data
            }
            convert($arg)
        };
        RESULT
    }};
}

#[cfg(test)]
mod tests {
    use super::hex;

    #[test]
    fn test_leading_zeros() {
        assert_eq!(hex!("01020304"), [1, 2, 3, 4]);
    }

    #[test]
    fn test_alphanumeric_lower() {
        assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    }

    #[test]
    fn test_alphanumeric_upper() {
        assert_eq!(hex!("E5E69092"), [0xE5, 0xE6, 0x90, 0x92]);
    }

    #[test]
    fn test_alphanumeric_mixed() {
        assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
    }
}
