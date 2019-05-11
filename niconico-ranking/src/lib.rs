extern crate reqwest;

use std::env;
use std::io::Error;

pub fn get_niconico_ranking() -> Result<String, Error> {

    let proxy = match env::var("http_proxy") {
        Ok(val) => Some(reqwest::Proxy::http(&val)),
        Err(_) => match env::var("https_proxy") {
            Ok(val) => Some(reqwest::Proxy::https(&val)),
            Err(_) => None
        }
    };

    let builder =  proxy.map_or(reqwest::Client::builder(), |p| reqwest::Client::builder().proxy(p.unwrap()));
    let client = builder.build().unwrap();

    return Ok(client.get("https://www.nicovideo.jp/ranking/fav/daily/all?rss=2.0&lang=ja-jp")
        .send().unwrap()
        .text().unwrap())
}
