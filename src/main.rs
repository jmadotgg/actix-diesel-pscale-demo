use actix_diesel_pscale_demo::routes;
use actix_files::NamedFile;
use actix_web::{
    get, guard,
    web::{self, ReqData},
    App, Error, HttpResponse, HttpServer, Responder,
};
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
    HttpServer::new(move || {
        App::new()
            .service(web::scope("api").configure(api_config))
            .service(index)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(addr)?
    .run()
    .await
}

#[get("/")]
async fn index() -> Result<NamedFile, Error> {
    let path = NamedFile::open("static/index.html")?;
    Ok(path)
}
