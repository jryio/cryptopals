/// Set 1 - Challenge 5
/// Repeating XOR
#[allow(dead_code)]
pub fn repeating_key_xor(plaintext: String, key: String) -> String {
    let mut result: Vec<u8> = Vec::with_capacity(plaintext.len());
    // We need to XOR each byte of the plaintext with each byte of the key modulo key_size
    for (i, b) in plaintext.bytes().enumerate() {
        let xor_single = b ^ (key.as_bytes()[i % key.len()]);
        result.push(xor_single);
    }
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use crate::one::challenge_5::repeating_key_xor;

    #[test]
    fn test_challenge_5() {
        let input =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".into();
        let key = "ICE".into();
        let output = repeating_key_xor(input, key);
        let answer = String::from(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        );
        assert_eq!(output, answer);
    }
}
