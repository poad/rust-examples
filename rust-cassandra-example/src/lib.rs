use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Comment {
    id: Option<String>,
    name: String,
    message: String
}

pub mod handlers {
    use tide::{Response, StatusCode};
    use cdrs::cluster::session::Session;
    use cdrs::load_balancing::RoundRobin;
    use cdrs::cluster::{TcpConnectionsManager, TcpConnectionPool};
    use cdrs::authenticators::NoneAuthenticator;
    use crate::Comment;

    type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

    pub async fn get(_: *CurrentSession) -> Result<Response, std::io::Error> {
        let comment = Comment {
            id: None,
            name: "test".into(),
            message: "Hello World!".into(),
        };

        return Ok(Response::new(StatusCode::Ok).body_json(&comment)?);
    }
}
