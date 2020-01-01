pub mod github {
    use actix_web::HttpRequest;
    use std::env;

    pub struct GitHub {}

    impl GitHub {
        pub fn new() -> Self {
            GitHub {}
        }

        pub async fn release_feed(
            &self, _req: HttpRequest,
        ) -> Result<String, reqwest::Error> {
            let proxy = match env::var("http_proxy") {
                Ok(val) => Some(reqwest::Proxy::http(&val)),
                Err(_) => match env::var("https_proxy") {
                    Ok(val) => Some(reqwest::Proxy::https(&val)),
                    Err(_) => None
                }
            };

            let builder = proxy.map_or(reqwest::Client::builder(), |p| reqwest::Client::builder().proxy(p.unwrap()));
            let client = builder.build().unwrap();

            client
                .get("https://github.com/rust-lang/rust/releases.atom")
                .send()
                .await?
                .text()
                .await
        }
    }
}