extern crate url;

use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use rand::{Rng, thread_rng};
use rustc_serialize::base64;
use crypto::mac::{Mac, MacResult};
use rustc_serialize::base64::ToBase64;
use time::now_utc;

fn encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>()
}

fn hmac_sha1(key: &[u8], data: &[u8]) -> MacResult {
    let mut hmac = Hmac::new(Sha1::new(), key);
    hmac.input(data);
    hmac.result()
}

fn signature(method: &str, uri: &str, query: &str, consumer_secret: &str, access_secret: &str) -> String {
    let base = format!("{}&{}&{}", encode(method), encode(uri), encode(&query));
    let key = format!("{}&{}", encode(consumer_secret), encode(access_secret));
    let conf = base64::Config {
        char_set: base64::CharacterSet::Standard,
        newline: base64::Newline::LF,
        pad: true,
        line_length: None,
    };
    hmac_sha1(key.as_bytes(), base.as_bytes()).code().to_base64(conf)
}

pub fn make_query(list: &Vec<(&str, &str)>, separator: &str) -> String {
    let mut result = String::from("");
    for item in list {
        if "" != result {
            result.push_str(separator);
        }
        result.push_str(&format!("{}={}", item.0, item.1));
    }
    result
}

pub fn make_oauth(
    consumer_key: &str,
    consumer_secret: &str,
    access_key: &str,
    access_secret: &str,
    method: &str,
    uri: &str,
    option: Option<&Vec<(&str, &str)>>
) -> String {
    let timestamp = format!("{}", now_utc().to_timespec().sec);
    let nonce = thread_rng().gen_ascii_chars().take(32).collect::<String>();
    let params0: Vec<(&str, &str)> = vec![
        ("oauth_consumer_key", consumer_key),
        ("oauth_signature_method", "HMAC-SHA1"),
        ("oauth_timestamp", &timestamp),
        ("oauth_version", "1.0a"),
        ("oauth_nonce", &nonce),
        ("oauth_token", access_key)
    ];
    let mut params1 = params0.clone();
    for i in option.iter().flat_map(|v| v.iter()) {
        params1.push((i.0, i.1));
    }
    params1.sort();
    let query = make_query(&params1, "&");
    let sign = encode(&signature(method, uri, &query, consumer_secret, access_secret));
    let mut params2 = params0;
    params2.push(("oauth_signature", &sign));
    make_query(&params2, ", ")
}