use dotenv::dotenv;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| match dotenv() {
        Ok(_) => println!("Loaded env solutions"),
        Err(e) => eprintln!("{:?}", e),
    })
}
