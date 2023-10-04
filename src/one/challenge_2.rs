use crate::Result;
/// Set 1 - Challenge 2
#[allow(dead_code)]
pub fn two_fixed_xor(input1: String, input2: String) -> Result<String> {
    let raw_1: Vec<u8> = hex::decode(input1)?;
    let raw_2: Vec<u8> = hex::decode(input2)?;

    let xor_result: Vec<u8> = raw_1
        .iter()
        .zip(raw_2.iter())
        .map(|(&r1, &r2)| r1 ^ r2)
        .collect();

    Ok(hex::encode(xor_result))
}

#[cfg(test)]
mod tests {
    use crate::one::challenge_2::two_fixed_xor;

    // Set 1 - Challenge 2
    #[test]
    fn test_challenge_2() {
        let input1: String = "1c0111001f010100061a024b53535009181c".into();
        let input2: String = "686974207468652062756c6c277320657965".into();
        let output: String = "746865206b696420646f6e277420706c6179".into();

        let result = two_fixed_xor(input1, input2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), output);
    }
}
