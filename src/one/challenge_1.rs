use crate::Result;
use base64::{engine::general_purpose, Engine as _};

/// Set 1 - Challenge 1
///
/// Convert Hex to Base64
///
/// Hexidecimal is base16 (0-9 + a-f)
/// Base64 encoding is base64 (a-z A-Z 0-9 '=', '+', '/')
pub fn one_hex_to_base64(input: String) -> Result<String> {
    let raw_bytes = hex::decode(input)?;
    let encoded_base_64_string = general_purpose::STANDARD_NO_PAD.encode(raw_bytes);
    Ok(encoded_base_64_string)
}

#[cfg(test)]
mod tests {
    use crate::one::challenge_1::one_hex_to_base64;
    // Set 1 - Challenge 1

    #[test]
    fn test_test_base64() {
        let input : String = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".into();
        let output: String =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".into();
        let result = one_hex_to_base64(input);
        assert!(result.is_ok());
        let encoded_base_64 = result.unwrap();
        assert_eq!(encoded_base_64, output);
    }
}
