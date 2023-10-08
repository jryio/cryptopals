use crate::one::challenge_3::three_single_xor_cipher;
use crate::utils::frequency_score;
use crate::Result;

/// Set 1 - Challenge 4
#[allow(dead_code)]
pub fn detect_single_char_xor(input: String) -> Result<String> {
    let mut maybe_plaintexts: Vec<String> = Vec::new();
    for line in input.lines() {
        if line.len() != 60 {
            continue;
        }
        // println!("line = {:?}", line);
        // This can fail, if it does, it may not be valid UTF8
        if let Ok(maybe_text) = three_single_xor_cipher(line.into()) {
            maybe_plaintexts.push(maybe_text);
        }
    }

    let mut best_score = std::u32::MAX;
    let mut highest_plaintext: String = "".into();
    for p in maybe_plaintexts {
        let score = frequency_score(p.as_bytes());
        if score < best_score {
            best_score = score;
            highest_plaintext = p;
        }
    }

    Ok(highest_plaintext)
}

#[cfg(test)]
mod tests {
    use super::detect_single_char_xor;
    use crate::env;

    #[test]
    fn test_challenge_4() {
        env::init();
        let input = include_str!("4.txt");
        let output = detect_single_char_xor(input.into());
        let answer = std::env::var("S1C4_ANS").expect(
            "Set 1 - Challenge 4 - Missing environment variable 'S1C4_ANS' containing solutioln",
        );
        assert!(output.is_ok());
        let output = output.unwrap();
        assert_eq!(output, answer);
    }
}
