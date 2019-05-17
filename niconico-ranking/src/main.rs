#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use actix_web::{server, App, HttpRequest, Responder};

use niconico_ranking::*;

use serde_xml_rs::from_reader;


#[derive(Debug, Deserialize)]
struct Item {
    pub title: String,
//    #[serde(rename = "link")]
//    pub link_item: String,
    pub guid: String,
//    pub pub_date: String,
    pub description: String
}

#[derive(Debug, Deserialize)]
struct Channel {
    pub title: String,
//    #[serde(rename = "link")]
//    pub link_channel: String,
    pub description: String,
//    pub pub_date: String,
//    pub last_build_date: String,
    pub generator: String,
    pub language: String,
    pub copyright: String,
    pub docs: String,
    #[serde(rename = "item", default)]
    pub items: Vec<Item>
}

#[derive(Debug, Deserialize)]
struct Rss {
    #[serde(rename = "channel")]
    pub channel: Channel
}

#[derive(Serialize, Deserialize)]
struct Keywords {
    items: Vec<String>,
}

struct KeywordsHandler;

fn handle(_: &HttpRequest) -> impl Responder {
    let rss = parse_xml(get_niconico_ranking().unwrap().to_owned());
    let keywords = Keywords {
        items: rss.channel.items.iter().map(|item| item.title.to_owned()).collect::<Vec<String>>()
    };
    let response = serde_json::to_string(&keywords).unwrap();
    return response;
}

fn main() {
    server::new(||{
        App::new()
            .resource("/", |r| r.f(handle))
    }).bind("0.0.0.0:8000");

    info!("On 8000");
}

fn parse_xml(src: String) -> Rss {
    return from_reader(src.as_bytes()).unwrap();
}