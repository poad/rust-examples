extern crate cdrs;

use std::env;

use async_std::main;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::cluster::session::{new as new_session};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use rust_cassandra_example::handlers;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let host = format!("{}:{}", env::var("DATABASE_URL").expect("DATABASE_URL must be set"), 9042);
    let node = NodeTcpConfigBuilder::new(&host, NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS test WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    no_compression.query(create_ks).expect("Keyspace create error");

    let mut app = tide::new();
    app.at("/").get(|_| async { handlers::get(&no_compression) });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
