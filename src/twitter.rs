extern crate json;
extern crate hyper;
use oauth;
use std::io::Read;

pub struct Twitter<'a> {
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
    pub access_key: &'a str,
    pub access_secret: &'a str,
}

impl<'a> Twitter<'a> {
    pub fn new(
        consumer_key: &'a str,
        consumer_secret: &'a str,
        access_key: &'a str,
        access_secret: &'a str,
    ) -> Twitter<'a> {
        Twitter {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_key: access_key,
            access_secret: access_secret,
        }
    }

    fn execute(&self, uri: &str, method: &str, option: Option<&Vec<(&str, &str)>>) -> json::JsonValue {
        let sign = oauth::make_oauth(
            &self.consumer_key,
            &self.consumer_secret,
            &self.access_key,
            &self.access_secret,
            &method,
            &uri,
            option
        );
        let header = format!("OAuth {}", sign);
        let mut res = String::new();
        if "GET" == method {
            let calced_uri = match option {
                Some(opt) => format!("{}?{}", uri, oauth::make_query(opt, "&")),
                None => String::from(uri)
            };
            hyper::Client::new().get(&calced_uri)
                .header(hyper::header::Authorization(header.to_owned()))
                .send()
                .unwrap()
                .read_to_string(&mut res)
                .unwrap();
        } else {
            let calced_body = match option {
                Some(opt) => oauth::make_query(opt, "&"),
                None => String::new()
            };
            let content: hyper::mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();
            hyper::Client::new().post(uri)
                .header(hyper::header::Authorization(header.to_owned()))
                .header(hyper::header::ContentType(content))
                .body(calced_body.as_bytes())
                .send()
                .unwrap()
                .read_to_string(&mut res)
                .unwrap();
        }
        
        json::parse(&res).unwrap()
    }

    pub fn verify_credentials(&self, option: Option<&Vec<(&str, &str)>>) -> json::JsonValue {
        let uri = "https://api.twitter.com/1.1/account/verify_credentials.json";
        let method = "GET";
        self.execute(&uri, &method, option)
    }

    pub fn statuses_update(&self, status: &str, option: Option<&Vec<(&str, &str)>>) -> json::JsonValue {
        let uri = "https://api.twitter.com/1.1/statuses/update.json";
        let method = "POST";
        let mut params = vec![("status", status)];
        for i in option.iter().flat_map(|v| v.iter()) {
            params.push((i.0, i.1));
        }
        self.execute(&uri, &method, Some(&params))
    }
}