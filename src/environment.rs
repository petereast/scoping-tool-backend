// Provide helper functions for getting environment variable, or using a default instead
use std::env;

pub fn redis_url() -> String {
    match env::var("redis_url") {
        Ok(v) => v,
        Err(_) => String::from("localhost"),
    }
}
