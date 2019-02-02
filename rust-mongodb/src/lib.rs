use mongodb::{bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::results::InsertOneResult;
use bson::Document;
use mongodb::error::Result;
use mongodb::coll::results::DeleteResult;

pub fn connect_mongodb() -> Client {
    return Client::connect("mongo", 27017)
        .expect("Failed to initialize standalone client.");
}

pub fn create_post(client: &Client, title: &str, body: &str) -> InsertOneResult {
    let coll = client.db("posts").collection("post");
    coll.insert_one(doc!{ "title": title, "body": body }, None).unwrap()
}

pub fn all_posts(client: Client) -> Vec<Document> {
    let coll = client.db("posts").collection("post");
    let cursor = coll.find(None, None).unwrap();
    let posts: Vec<Document> = cursor
        .filter(|i| {
            match i {
                Ok(_) => true,
                _ => false
            }
        })
        .map(|result| {
            match result {
                Ok(item) => item,
                _ => panic!("")
            }
        }).collect();
    return posts;
}

pub fn get_post(client: &Client, title: String) -> Option<Document> {
    let coll = client.db("posts").collection("post");
    return match coll.find_one(Some(doc!{"title": title}), None) {
            Ok(item) => item,
            _ => None
        };
}

pub fn delete_post(client: &Client, title: String) -> Result<DeleteResult> {
    let coll = client.db("posts").collection("post");
    return coll.delete_one(doc!{"title": title.to_owned()}, None);
}
