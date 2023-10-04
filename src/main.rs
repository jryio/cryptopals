mod env;
mod one;
mod utils;

#[allow(dead_code)]
pub type AnyError = Box<dyn std::error::Error>;
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, AnyError>;

fn main() {

    // unimplemented!()
}
