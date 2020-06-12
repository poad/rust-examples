pub mod cassandra {

    use serde::{Deserialize, Serialize};
    use cdrs::authenticators::NoneAuthenticator;
    use cdrs::cluster::{NodeTcpConfigBuilder, ClusterTcpConfig};
    use cdrs::cluster::session::{new as new_session};
    use cdrs::load_balancing::RoundRobin;
    use async_trait::async_trait;
    use cdrs::query::QueryExecutor;

    #[derive(Deserialize, Serialize)]
    pub struct Comment {
        pub id: Option<String>,
        pub name: String,
        pub message: String
    }

    pub struct Client {
        host: String
    }

    #[async_trait]
    pub trait CassandraClient {
        fn new(host: String) -> Client;

        fn create_keyspace(&self, name: String);
    }

    impl CassandraClient for Client {
        fn new(host: String) -> Client {
            Client {
                host
            }
        }

        fn create_keyspace(&self, name: String) {
            let node = NodeTcpConfigBuilder::new(&self.host, NoneAuthenticator {}).build();
            let cluster_config = ClusterTcpConfig(vec![node]);
            let session =
                new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

            let create_ks = format!(
                "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {};",
                name,
                "{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }"
            );
            session.query(create_ks).expect("Keyspace create error");
        }
    }
}