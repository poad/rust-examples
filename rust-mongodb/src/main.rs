extern crate bodyparser;
#[macro_use]
extern crate iron;
extern crate persistent;
extern crate router;
extern crate rust_mongodb;
#[macro_use]
extern crate serde_derive;
extern crate urlencoded;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate percent_encoding;

use iron::mime::*;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::Router;

use self::rust_mongodb::*;
use percent_encoding::percent_decode;

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;


fn main() {
    env_logger::init();

    let mut router = Router::new();                     // Alternative syntax:
    router.get("/", get, "index");        // let router = router!(index: get "/" => handler,
    router.get("/:title", get, "get");        //  query: get "/:query" => handler);

    let mut chain = Chain::new(post);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    router.post("/", chain, "create");

    let mut delete_chain = Chain::new(delete);
    delete_chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    router.delete("/", delete_chain, "delete");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}

fn get(req: &mut Request) -> IronResult<Response> {
    let client = connect_mongodb();

    let title = req.extensions.get::<Router>().unwrap().find("title");

    let results = match title {
        None => {
            trace!("title is empty");
            all_posts(client)
                .iter()
                .map(|doc| {
                    let title_value = match doc.get_str("title") {
                        Ok(title_value) => title_value,
                        _ => panic!("")
                    };
                    let body_value = match doc.get_str("body") {
                        Ok(body_value) => body_value,
                        _ => panic!("")
                    };
                    Post {
                        title: title_value.to_owned(),
                        body: body_value.to_owned()
                    }
                }).collect()
        },
        Some(title_value) => {
            let decoded = percent_decode(title_value.as_bytes()).decode_utf8().unwrap();

            trace!("title is {}", decoded);

            let post = get_post(&client, decoded.parse::<String>().expect("Invalid ID"));
            match post {
                Some(doc) => {
                    let decoded = match doc.get_str("title") {
                        Ok(decoded) => decoded,
                        _ => panic!("")
                    };
                    let body_value = match doc.get_str("body") {
                        Ok(body_value) => body_value,
                        _ => panic!("")
                    };
                    vec!(Post {
                        title: decoded.to_owned(),
                        body: body_value.to_owned()
                    })
                },
                None => {
                    let vec: Vec<Post> = Vec::new();
                    vec
                }
            }
        }
    };
    let response = itry!(serde_json::to_string(&results));
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, response)))
}

fn post(req: &mut Request) -> IronResult<Response> {
    let client = connect_mongodb();

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            match json_body.as_object() {
                Some(post) => {
                    let title = match post.get("title") {
                        Some(value) => match value.as_str() {
                            Some(v) => v,
                            None => ""
                        },
                        None => ""
                    };
                    let body = match post.get("body") {
                        Some(value) => match value.as_str() {
                            Some(v) => v,
                            None => ""
                        },
                        None => ""
                    };

                    let _post = create_post(&client, title, body);
                    let res_post = Post {
                        title: title.to_owned(),
                        body: body.to_owned(),
                    };
                    let response = itry!(serde_json::to_string(&res_post));
                    let content_type = "application/json".parse::<Mime>().unwrap();

                    Ok(Response::with((content_type, status::Ok, response)))
                }
                None => Ok(Response::with(status::BadRequest))
            }
        }
        Ok(None) => Ok(Response::with(status::BadRequest)),
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(Response::with(status::BadRequest))
        }
    }
}

fn delete(req: &mut Request) -> IronResult<Response> {
    let client = connect_mongodb();

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => {
            match json_body.as_object() {
                Some(post) => {
                    let title = match post.get("title") {
                        Some(value) => match value.as_str() {
                            Some(v) => v,
                            None => ""
                        },
                        None => ""
                    };

                    match delete_post(&client, title.parse::<String>().expect("Invalid ID")) {
                        Ok(_) => Ok(Response::with(status::Ok)),
                        Err(err) => {
                            println!("Error: {:?}", err);
                            Ok(Response::with(status::BadRequest))
                        }
                    }
                }
                None => Ok(Response::with(status::BadRequest))
            }
        }
        Ok(None) => Ok(Response::with(status::BadRequest)),
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(Response::with(status::BadRequest))
        }
    }
}