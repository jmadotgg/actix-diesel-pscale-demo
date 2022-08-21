use actix_diesel_pscale_demo::routes;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::org::api_config;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let addr = (
        (&env::var("HOSTNAME").expect("HOSTNAME not set")[..]),
        env::var("PORT")
            .expect("PORT not set")
            .parse::<u16>()
            .expect("Invalid port"),
    );
    HttpServer::new(move || App::new().service(web::scope("api").configure(api_config)))
        .bind(addr)?
        .run()
        .await
}
