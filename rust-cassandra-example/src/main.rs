use std::env;

use rust_cassandra_example::cassandra::{Comment, Client, CassandraClient};
use tide::Server;
use std::sync::RwLock;


pub struct State {
    client: Client,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let host = format!("{}:{}", env::var("DATABASE_URL").expect("DATABASE_URL must be set"), 9042);
    let client = Client::new(host);
    client.create_keyspace("test".parse().unwrap());
    client.create_table("test.test".parse().unwrap(), "(id text PRIMARY KEY, name text, message text)".parse().unwrap());

    let mut app = Server::with_state(State {
        client,
    });
    app.at("/").get( |_| async move {
        let comment = Comment {
            id: None,
            name: "test".into(),
            message: "Hello World!".into(),
        };

        let mut res = tide::Response::new(tide::StatusCode::Ok);
        res.set_body(tide::Body::from_json(&comment).unwrap());
        Ok(res)
    });
    app.listen("0.0.0.0:8080").await.unwrap();
    Ok(())
}

async fn get(req: tide::Request<State>) {

    let client = &req.state().client;
    client.select("SELECT * from test.test".parse().unwrap());
}