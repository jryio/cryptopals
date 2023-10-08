use crate::utils::{frequency_score, xor_single_byte};
use crate::Result;
/// Set 1 - Challenge 3
pub fn three_single_xor_cipher(input: String) -> Result<String> {
    let input_bytes = hex::decode(input)?;
    let (_key, output_bytes) = break_single_char_xor_cipher(input_bytes.as_slice());
    Ok(String::from_utf8(output_bytes)?)
}

pub fn break_single_char_xor_cipher(input: &[u8]) -> (u8, Vec<u8>) {
    let mut highest_score = 0.0;
    let mut maybe_key = 0;
    for i in 0..255 {
        let xor_result = xor_single_byte(input, i);
        if !xor_result.is_ascii() {
            continue;
        }

        if let Ok(string) = std::str::from_utf8(&xor_result) {
            // Some results of XORing the input with a single character produce non-ascii values.
            // Therefore we want to look for letters, numbers, and punc
            if string.chars().all(|c| {
                c.is_ascii_alphabetic()
                    || c.is_whitespace()
                    || c == '.'
                    || c == '\''
                    || c == '!'
                    || c == '?'
                    || c == '\n'
            }) {
                let score = frequency_score(xor_result.as_slice());
                println!("challenge_3 score={score} highest_score={highest_score} maybe_key = {i}");

                if score >= highest_score {
                    highest_score = score;
                    maybe_key = i;
                }
            }
        }
    }

    (maybe_key, xor_single_byte(input, maybe_key))
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
            "Set 1 - Challange 3 - Missing environment variable 'S1C3_ANS' containing solution",
        );

        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, answer);
    }
}
