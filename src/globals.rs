use worker::*;

pub const URL_KV: &str = "URLS";
pub const REVERSE_URL_KV: &str = "REVERSE_URLS";
pub const URL_KEY: &str = "url";
pub const STUB_KEY: &str = "stub";

pub fn default_redirect() -> Result<Response> {
    Response::redirect(Url::parse("https://jiahua.io").unwrap())
}

pub fn missing_redirect() -> Result<Response> {
    Response::redirect(Url::parse("https://jiahua.io/404").unwrap())
}
