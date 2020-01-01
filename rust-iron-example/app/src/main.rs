extern crate bodyparser;
extern crate diesel;
#[macro_use]
extern crate iron;
extern crate persistent;
extern crate router;
extern crate rust_iron_example;
#[macro_use]
extern crate serde_derive;
extern crate urlencoded;


use iron::mime::*;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::Router;
use urlencoded::UrlEncodedQuery;

use self::rust_iron_example::*;

#[derive(Serialize, Deserialize)]
struct ResPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;


fn main() {
    let mut router = Router::new();           // Alternative syntax:
    router.get("/", get, "index");        // let router = router!(index: get "/" => handler,
    router.get("/:id", get, "id");  //                      query: get "/:query" => handler);

    let mut chain = Chain::new(post);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    router.post("/", chain, "create");

    router.delete("/:id", delete, "id");

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}

fn get(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();

    let id = req.extensions.get::<Router>().unwrap().find("id");

    let results = match id {
        None => {
            let include_unpublished = match req.get_ref::<UrlEncodedQuery>() {
                Ok(ref params) => {
                    params.get("include_unpublished")
                        .map_or(false, |values| values
                            .first()
                            .map_or(false, |value| value.eq("true")),
                        )
                }
                Err(ref e) => {
                    println!("{:?}", e);
                    false
                }
            };
            all_posts(connection, include_unpublished)
        }
        Some(id_value) => vec!(get_post(&connection, id_value.parse::<i32>().expect("Invalid ID")))
    };
    let response = itry!(serde_json::to_string(&results
            .iter()
            .map(|result| {
                let r = ResPost {
                    id: result.id.to_owned(),
                    title: result.title.to_owned(),
                    body: result.body.to_owned(),
                    published: result.published.to_owned()
                };
                r
            })
            .collect::<Vec<ResPost>>()));
    let content_type = "application/json".parse::<Mime>().unwrap();

    Ok(Response::with((content_type, status::Ok, response)))
}

fn post(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();

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

                    let post = create_post(&connection, title, &body);
                    let res_post = ResPost {
                        id: post.id,
                        title: post.title,
                        body: post.body,
                        published: post.published,
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
    let connection = establish_connection();

    let id = req.extensions.get::<Router>().unwrap().find("id");

    match id {
        None => {
            Ok(Response::with(status::BadRequest))
        }
        Some(id_value) => {
            match delete_post(&connection, id_value.parse::<i32>().expect("Invalid ID")) {
                Ok(_) => Ok(Response::with(status::Ok)),
                Err(err) => {
                    println!("Error: {:?}", err);
                    Ok(Response::with(status::BadRequest))
                }
            }

        }
    }
}
