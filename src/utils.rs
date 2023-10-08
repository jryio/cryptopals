use std::collections::HashMap;

/// Computes the XOR of every byte in `buffer` against `single`
pub fn xor_single_byte(buffer: &[u8], single: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(buffer.len());
    for b in buffer.iter() {
        result.push(b ^ single);
    }
    result
}

#[allow(dead_code)]
pub fn xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    assert_eq!(
        first.len(),
        second.len(),
        "XOR requires that both buffers are of equal length. Got left = {} right = {}",
        first.len(),
        second.len()
    );

    first
        .iter()
        .zip(second.iter())
        .map(|(f, s)| f ^ s)
        .collect()
}

pub fn xor_repeat(source: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(source.len());
    for chunk in source.chunks(key.len()) {
        // A chunk may not always be target.len() if target.len() does not evenly divide source.len().
        let chunk_len = chunk.len();
        for (c, d) in chunk.iter().zip(key[..chunk_len].iter()) {
            result.push(c ^ d);
        }
    }
    result
}

static ASCII_CONTROL_END: u8 = 0x20;
static ASCII_CONTROL_DEL: u8 = 0x7F;
static ASCII_NEWLINE: u8 = b'\n';
static ASCII_WHITESPACE: u8 = b' ';
static ASCII_TAB: u8 = b'\t';
static ASCII_PERIOD: u8 = b'.';
static DEFAULT_FREQUENCY: f64 = 0.0;
static ENGLISH_FREQUENCIES: [(u8, f64); 28] = [
    (b'a', 6.09),
    (b'b', 1.05),
    (b'c', 2.84),
    (b'd', 2.92),
    (b'e', 11.36),
    (b'f', 1.79),
    (b'g', 1.38),
    (b'h', 3.41),
    (b'i', 5.44),
    (b'j', 0.24),
    (b'k', 0.41),
    (b'l', 2.92),
    (b'm', 2.76),
    (b'n', 5.44),
    (b'o', 6.00),
    (b'p', 1.95),
    (b'q', 0.24),
    (b'r', 4.95),
    (b's', 5.68),
    (b't', 8.03),
    (b'u', 2.43),
    (b'v', 0.97),
    (b'w', 1.38),
    (b'x', 0.24),
    (b'y', 1.30),
    (b'z', 0.03),
    (b' ', 12.17),
    (b'.', 6.57), // Punctuation characters
];

/// Computes the frequency of characters in an buffer and compares against known English langauge
/// character frequency, returning a score.
///
/// The score is computed using the chi-sqaured test: (difference squared of the actual - expected
/// values) divided by the expected value, summed for all measurements
///
/// Low values of chi-sqaured indicate a high fit between the recorded results and the expected.
/// High score indicate large difference between actual and expected results.
pub fn frequency_score(buffer: &[u8]) -> u32 {
    // Highest score says that this is invalid
    if !buffer.is_ascii() {
        return std::u32::MAX;
    }

    // Highest score says that this is invalid
    if buffer
        .iter()
        .any(|&c| c != ASCII_NEWLINE && (c < ASCII_CONTROL_END || c == ASCII_CONTROL_DEL))
    {
        return std::u32::MAX;
    }

    let actual_chars_len = buffer.len() as f64;
    let actual_chars_count: HashMap<u8, f64> = buffer
        .iter()
        .map(|&x| (x as char).to_ascii_lowercase())
        .fold(HashMap::new(), |mut hash_map, c| {
            // ASCII a-z or A-Z
            let key: u8 = if c.is_alphabetic() {
                c as u8
            }
            // whitespace mapping
            else if c as u8 == ASCII_WHITESPACE || c as u8 == ASCII_TAB {
                ASCII_WHITESPACE
            }
            // Convert all other characters (punctuation or numbers) to '.'
            else {
                ASCII_PERIOD
            };

            hash_map
                .entry(key)
                .and_modify(|f| *f += 1.0)
                .or_insert(DEFAULT_FREQUENCY);

            hash_map
        });

    let mut chi_sqrd = 0.0;
    for (c, f) in ENGLISH_FREQUENCIES {
        let expect_num_char = (f / 100.0) * actual_chars_len;
        let actual_num_char = actual_chars_count.get(&c).unwrap_or(&DEFAULT_FREQUENCY);
        let diff = expect_num_char - actual_num_char;
        // println!(
        //     "char = {c} expected = {expect_num_char} actual = {actual_num_char} diff = {diff}"
        // );
        chi_sqrd += (diff * diff) / expect_num_char;
    }
    // println!("chi-sqaured = {chi_sqrd}");
    chi_sqrd as u32

    // println!("Expected Frequencies {:?}", expected_frequencies);
    // println!("Actual Frequencies {:?}", actual_frequenies);
}

/// Hamming Distance is a count of the number of differing bits between two inputs
#[inline]
pub fn hamming_distance(first: &[u8], second: &[u8]) -> usize {
    assert_eq!(first.len(), second.len(), "Lengths of both strings in Hamming Distance must be equal. first.len() = {}, second.len() = {}", first.len(), second.len());
    first
        .iter()
        .zip(second.iter())
        .fold(0, |score, (x, y)| score + (x ^ y).count_ones() as usize)
}

/// Hamming distance computed only on one input buffer broken into `N` chunks and compared pairwise
pub fn normalized_hamming_distance(input: &[u8], chunk_size: usize, num_blocks: usize) -> f64 {
    let chunks: Vec<&[u8]> = input.chunks(chunk_size).take(num_blocks).collect();
    let mut score: f64 = 0.0;
    for i in 0..num_blocks {
        for j in i..num_blocks {
            score += hamming_distance(chunks[i], chunks[j]) as f64
        }
    }

    score / chunk_size as f64
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::hamming_distance;

    #[test]
    fn test_hamming_distance() {
        let case_one = (String::from(""), String::from(""), 0);
        let case_two = (String::from("same"), String::from("same"), 0);
        let case_three = (
            String::from("this is a test"),
            String::from("wokka wokka!!!"),
            37,
        );
        let all_cases = vec![case_one, case_two, case_three];

        for (first, second, output) in all_cases {
            let result = hamming_distance(first.as_bytes(), second.as_bytes());
            assert_eq!(result, output);
        }
    }
}
