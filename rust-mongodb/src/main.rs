extern crate env_logger;
#[macro_use]
extern crate log;
extern crate percent_encoding;
extern crate serde_derive;
extern crate urlencoded;

use percent_encoding::percent_decode;
use tide::{Request, Response, StatusCode};
use tide::prelude::*;

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    let mut app = tide::new();
    app.at("/").get(index);
    app.at("/{}").get(get);
    app.at("/").post(create);
    app.at("/").delete(delete);

    app.listen("0.0.0.0:3000").await?;
    Ok(())
}

async fn index(_: Request<()>) -> tide::Result {
    let client = rust_mongodb::connect_mongodb();

    let posts = rust_mongodb::all_posts(client).await;
    let results: Vec<Post> = posts
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
                body: body_value.to_owned(),
            }
        }).collect();

    Ok(json!(results).into())
}

async fn get(mut req: Request<()>) -> tide::Result {
    let client = rust_mongodb::connect_mongodb();

    let body: Post = req.body_json().await?;

    let decoded = percent_decode(body.title.as_bytes()).decode_utf8().unwrap();

    trace!("title is {}", decoded);

    let post = rust_mongodb::get_post(&client, decoded.parse::<String>().expect("Invalid ID")).await;

    let results = match post {
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
                body: body_value.to_owned(),
            })
        },
        None => {
            let vec: Vec<Post> = Vec::new();
            vec
        }
    };

    Ok(json!(results).into())
}

async fn create(mut req: Request<()>) -> tide::Result {
    let client = rust_mongodb::connect_mongodb();

    let post: Post = req.body_json().await?;
    let title = post.title;
    let body = post.body;

    rust_mongodb::create_post(&client, &title, &body).await;

    Ok(json!(Post { title: title.to_owned(), body: body.to_owned(), }).into())
}

async fn delete(mut req: Request<()>) -> tide::Result {
    let client = rust_mongodb::connect_mongodb();

    let json_body: tide::Result<Post> = req.body_json().await;
    match json_body {
        Ok(post) => {
            match rust_mongodb::delete_post(&client, post.title.parse::<String>().expect("Invalid ID")).await {
                Ok(_) => Ok(Response::new(StatusCode::Ok)),
                Err(err) => {
                    println!("Error: {:?}", err);
                    Ok(Response::new(StatusCode::BadRequest))
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(Response::new(StatusCode::BadRequest))
        }
    }
}