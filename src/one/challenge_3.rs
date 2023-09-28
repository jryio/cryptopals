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
            // We should not perform frequency analysis on these outputs.
            if string.is_ascii() {
                println!("string(xor_result) for key = {i} - {string:?}");
                let score = frequency_score(xor_result.as_slice());
                // println!("score = {}, key = {}, plaintext = {}", score, i, string);

                if score > highest_score {
                    highest_score = score;
                    maybe_key = i;
                }
            }
        }
    }

    let maybe_plaintext_buf = xor_single_byte(input_bytes.as_slice(), maybe_key);
    let maybe_plaintext = std::str::from_utf8(maybe_plaintext_buf.as_slice())?;

    println!(
        "highest_score = {}, key = {}, plaintext = {}",
        highest_score, maybe_key, maybe_plaintext
    );

    Ok("".into())
}
#[cfg(test)]
mod tests {
    use crate::one::challenge_3::three_single_xor_cipher;

    // Set 1 - Challenge 3
    #[test]
    fn test_three_single_xor_cipher() {
        let input: String =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".into();
        let result = three_single_xor_cipher(input);
    }
}
