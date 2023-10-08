use crate::{
    one::challenge_3::break_single_char_xor_cipher,
    utils::{frequency_score, normalized_hamming_distance, xor_repeat},
};

fn candiate_keysizes(input: &[u8], num_keys: usize) -> Vec<usize> {
    let mut key_hamming_distances: Vec<(usize, f64)> = (2..40)
        .map(|size| (size, normalized_hamming_distance(input, size, 4)))
        .collect();
    key_hamming_distances
        .sort_by(|&(_, score1), &(_, score2)| score1.partial_cmp(&score2).unwrap());
    println!("All key sizes by normalized hamming distance = {key_hamming_distances:?}");
    key_hamming_distances
        .iter()
        .take(num_keys)
        .map(|(key_size, _)| *key_size)
        .collect()
}

fn transpose_blocks(input: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let blocks: Vec<Vec<u8>> = input.chunks_exact(keysize).map(Vec::from).collect();

    // For each byte `i` of a cipher text block, there will be a Vec to store all `i` bytes in each
    // block
    //
    // Therefore there are `i` vecs each with size `block.len()`
    let mut ith_bytes: Vec<Vec<u8>> = vec![vec![0u8; blocks.len()]; keysize];
    for (b, block) in blocks.iter().enumerate() {
        for (i, byte) in block.iter().enumerate() {
            ith_bytes[i][b] = *byte;
        }
    }

    ith_bytes
}

/// Returns a Vec of possible single byte keys for each transposed block
///
/// Each single byte key in a Vec<u8> makes the whole key
fn break_single_xor_block(input: &[u8], keysize: usize) -> Vec<u8> {
    let transposed = transpose_blocks(input, keysize);
    transposed
        .iter()
        .map(|block| break_single_char_xor_cipher(block))
        .map(|(key, _output)| key)
        .collect()
}

/// Set 1 - Challenge 6
///
/// Outline:
///
/// 0. We know that the cipher is created using a repeating XOR key of length `K`,
///    The length is unknown. Therefore we need to check all the lengths
///
/// 1. Test the cipher text for the correct keysize `K` by breaking the cipher text into 2 blocks
///    of length `K_1`. Compute the hamming distnace between the two blocks and divide the result
///    by keysize `K`.
///
///    The keysize `K_i` which produces the smallest normalized (divided by keysize `K_i`) is
///    likely the correct keylength `K`.
///
/// 2. With the knowledge of the key length, break the cipher text into blocks of length `K`, this
///    is a repeating XOR cipher so each block will be encrypted with exactly the same key.
///
/// 3. Take the first byte from every block, put this into a list called "first_bytes". Take the
///    second byte from every block, put this into a list called "second_bytes", repeat this for
///    all bytes in every block.
///
///    Now that we have a list of all of the first bytes of our cipher text blocks, we know they
///    all must be XORed with the same exact character of the key `K`.
///
///    Break this 'single character XOR cipher like you did in Challenge 3'.
///
/// 4. Repeat this single XOR cipher breaking for the remaining characters of the key `K` and the
///    associated list of bytes with those characters.
///
/// 5. Learn the key
#[allow(dead_code)]
pub fn break_repeating_xor(input: Vec<u8>) -> Vec<u8> {
    // let test = xor_repeat(&input, b"Terminator X: Bring the noise");
    // let test = String::from_utf8(test).unwrap();
    // println!("TEST = {test:?}");

    candiate_keysizes(&input, 3)
        .iter()
        .map(|maybe_keysize| break_single_xor_block(&input, *maybe_keysize))
        .min_by_key(|key| {
            println!(
                "key_len = {}, key = {key:?} key_string = {}",
                key.len(),
                String::from_utf8(key.clone()).unwrap()
            );
            let maybe_plaintext = xor_repeat(&input, key);
            // println!(
            //     "Maybe Plaintext = {}",
            //     String::from_utf8(maybe_plaintext.clone()).unwrap()
            // );
            let frequency_score = frequency_score(&maybe_plaintext.clone());
            println!("frequency_score = {frequency_score}");
            frequency_score as u32
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use data_encoding::BASE64_MIME;

    use crate::{env, one::challenge_6::break_repeating_xor};

    #[test]
    fn test_challenge_6() {
        env::init();

        let input = include_str!("6.txt");
        // Decode the cipher text from Base64 to Byte Slice
        let input = BASE64_MIME.decode(input.as_bytes());
        assert!(input.is_ok());
        let input = input.unwrap();
        let output = break_repeating_xor(input.into());
        let answer = std::env::var("S1C6_ANS").expect(
            "Set 1 - Challenge 5 - Missing environment variable 'S1C6_ANS' containing solutioln",
        );

        println!("output = {output:+?}");
        assert_eq!(output, answer.as_bytes());
    }
}
