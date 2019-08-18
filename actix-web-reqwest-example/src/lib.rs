pub mod github {
    use actix_web::{HttpRequest, HttpResponse};
    use actix_web::error::Error;
    use futures::Future;

    pub struct GitHub {
    }

    impl GitHub {
        pub fn new() -> Self {
            GitHub {}
        }

        pub fn release_feed(
            &self, _req: HttpRequest
        ) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
            Box::new(
                reqwest::r#async::Client::new()
                    .get("https://github.com/rust-lang/rust/releases.atom")
                    .send()
                    .map_err(|err| {
                        log::error!("Get Activities ExternalServiceError: {}", err);
                        Error::from(HttpResponse::InternalServerError().finish())
                    })
                    .and_then(|mut res| {
                        res.text()
                            .map_err(|err| {
                                log::error!("Get Activities ExternalServiceError: {}", err);
                                Error::from(HttpResponse::InternalServerError().finish())
                            })
                            .and_then(|r|
                                HttpResponse::Ok()
                                    .content_type("application/json")
                                    .body(r)
                            )
                    })
                    .and_then(|resp| futures::future::ok::<_, Error>(resp))
            )
        }
    }
}