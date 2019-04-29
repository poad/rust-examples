#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use iron::prelude::*;
use iron::status;
use iron::*;
use iron::mime::*;

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

use std::collections::HashSet;
use iron_cors::CorsMiddleware;

struct KeywordsHandler;

impl Handler for KeywordsHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let rss = parse_xml(get_niconico_ranking().to_owned());
        let keywords = Keywords {
            items: rss.channel.items.iter().map(|item| item.title.to_owned()).collect::<Vec<String>>()
        };
        let response = itry!(serde_json::to_string(&keywords));
        let content_type = "application/json".parse::<Mime>().unwrap();
        return Ok(Response::with((status::Ok, content_type, response)));
    }
}

fn main() {
    // Initialize handler
    let handler = KeywordsHandler {};

    let allowed_hosts = ["localhost:3000"].iter()
        .map(ToString::to_string)
        .collect::<HashSet<_>>();
//    let middleware = CorsMiddleware::with_whitelist(allowed_hosts);
    let middleware = CorsMiddleware::with_allow_any();

    // Setup chain with middleware
    let mut chain = Chain::new(handler);
    chain.link_around(middleware);

    let _server = Iron::new(chain).http("0.0.0.0:8000").unwrap();
    info!("On 8000");
}

fn parse_xml(src: String) -> Rss {
    return from_reader(src.as_bytes()).unwrap();
}