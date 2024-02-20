use std::env;

pub fn get() -> String {
    env::var("USER").unwrap()
}
