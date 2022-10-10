use rand::{distributions::Alphanumeric, prelude::*};
use worker::*;

pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

/**
 * Creates a random 5 character alphanumeric (including upper/lowercase) string
 */
pub fn create_stub() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .collect()
}
