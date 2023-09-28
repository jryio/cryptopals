mod one;
mod utils;

use one::challenge_3::*;

use crate::utils::xor_single_byte;

#[allow(dead_code)]
pub type AnyError = Box<dyn std::error::Error>;
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, AnyError>;

fn main() {
    unimplemented!()
}
