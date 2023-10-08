use std::{collections::HashMap, num};

/// Computes the XOR of every byte in `buffer` against `single`
pub fn xor_single_byte(buffer: &[u8], single: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(buffer.len());
    for b in buffer.iter() {
        result.push(b ^ single);
    }
    result
}

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

/// Computes the frequency of characters in an buffer and compares against known English langauge
/// character frequency, returning a score.
pub fn frequency_score(buffer: &[u8]) -> f64 {
    let default_frequency = 0.008;
    let english_letters = String::from("abcdefghijklmnopqrstuvwxyz");
    let english_frequency: [f64; 26] = [
        8.2,   /* A */
        1.5,   /* B */
        2.8,   /* C */
        4.3,   /* D */
        12.7,  /* E */
        2.2,   /* F */
        2.0,   /* G */
        6.1,   /* H */
        7.0,   /* I */
        0.15,  /* J */
        0.77,  /* K */
        4.0,   /* L */
        2.4,   /* M */
        6.7,   /* N */
        7.5,   /* O */
        1.9,   /* P */
        0.095, /* Q */
        6.0,   /* R */
        6.3,   /* S */
        9.1,   /* T */
        2.8,   /* U */
        0.98,  /* V */
        2.4,   /* W */
        0.15,  /* X */
        2.0,   /* Y */
        0.074, /* Z */
    ];

    let expected_frequencies: HashMap<char, f64> = english_letters
        .chars()
        .zip(english_frequency.iter())
        .fold(HashMap::new(), |mut hash_map, (c, f)| {
            hash_map.insert(c, *f / 100.0);

            hash_map
        });

    // println!("english_frequency = {expected_frequencies:#?}");

    let actual_chars_len = buffer.len() as f64;
    let actual_frequenies: HashMap<char, f64> = buffer
        .iter()
        .map(|&x| (x as char).to_ascii_lowercase())
        .fold(HashMap::new(), |mut hash_map, c| {
            hash_map
                .entry(c)
                .and_modify(|f| *f = (*f + 1.0) / actual_chars_len)
                .or_insert(0.0);

            hash_map
        });

    let mut chi_sqrd = 0.0;

    for (c, f) in actual_frequenies {
        let expected_num = match expected_frequencies.get(&c) {
            // In a 'perfect' string, how many of characters `c` would appear based on the
            // english_frequency `f`
            Some(f) => (*f) * (actual_chars_len),
            None => default_frequency,
        };

        let actual_num = f;
        let diff = actual_num - expected_num;

        if expected_num > 0.0 {
            chi_sqrd += (diff * diff) / expected_num;
        }
    }

    chi_sqrd

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
