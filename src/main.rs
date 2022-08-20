use actix_diesel_pscale_demo::routes;
use actix_web::{web, App, HttpServer};
use routes::org::api_config;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(web::scope("api").configure(api_config)))
        .bind((HOSTNAME, PORT))?
        .run()
        .await
}
