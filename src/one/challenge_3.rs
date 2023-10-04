use crate::utils::{frequency_score, xor_single_byte};
use crate::Result;
/// Set 1 - Challenge 3
pub fn three_single_xor_cipher(input: String) -> Result<String> {
    let input_bytes = hex::decode(input)?;

    let mut highest_score = 0.0;
    let mut maybe_key = 0;
    for i in 0..255 {
        let xor_result = xor_single_byte(&input_bytes, i);
        if let Ok(string) = std::str::from_utf8(&xor_result) {
            // Some results of XORing the input with a single character produce non-ascii values.
            // Therefore we want to look for letters, numbers, and punc
            if string.chars().all(|c| {
                c.is_ascii_alphabetic()
                    || c.is_whitespace()
                    || c == '.'
                    || c == '-'
                    || c == '\''
                    || c == '!'
                    || c == '?'
            }) {
                let score = frequency_score(xor_result.as_slice());

                if score > highest_score {
                    highest_score = score;
                    maybe_key = i;
                }
            }
        }
    }

    let maybe_plaintext_buf = xor_single_byte(input_bytes.as_slice(), maybe_key);
    let maybe_plaintext = std::str::from_utf8(maybe_plaintext_buf.as_slice())?;

    Ok(maybe_plaintext.into())
}
#[cfg(test)]
mod tests {
    use crate::{env, one::challenge_3::three_single_xor_cipher};

    // Set 1 - Challenge 3
    #[test]
    fn test_challenge_3() {
        env::init();

        let input: String =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into();
        let output = three_single_xor_cipher(input);

        let answer = std::env::var("S1C3_ANS").expect(
            "Set 1 - Challange 3 - Missing environment variable 'S1C3' containing solution",
        );

        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, answer);
    }
}
