use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use actix_web::http::header;
use serde::Serialize;




#[derive(Serialize)] // Add this derive attribute for JSON serialization
struct Message {
    message: String,
}

async fn index() -> impl Responder {
    let message = Message {
        message: "Hello world!".to_owned(),
    };
    HttpResponse::Ok().json(message) // Return the JSON response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Replace with the URL of your React front-end
            .allowed_methods(vec!["GET", "POST"]) // Add any additional HTTP methods your backend supports
            .allowed_headers(vec![header::CONTENT_TYPE])
            .max_age(3600); // Set an appropriate max age for caching

        App::new()
            .wrap(cors)
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
