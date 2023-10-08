use dotenv::dotenv;

use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

#[allow(dead_code)]
pub fn init() {
    INIT.call_once(|| match dotenv() {
        Ok(_) => println!("Loaded env solutions"),
        Err(e) => eprintln!("{:?}", e),
    })
}
