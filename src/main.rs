use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // allow any port on localhost
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    // set allowed methods list
                    .allowed_methods(vec!["GET"])
                    // set list of headers that are safe to expose
                    .expose_headers(&[header::CONTENT_DISPOSITION]),
            )
            .route("/", web::get().to(handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn handler() -> impl Responder {
    "Hello World!"
}
