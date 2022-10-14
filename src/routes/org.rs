use crate::dal::{
    self,
    org::{NewOrg, UpdatedOrg},
};
use actix_web::{delete, get, http::header::ContentType, post, put, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Filter {
    org_name: String,
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(org_create)
        .service(org_read)
        .service(org_update)
        .service(org_delete)
        .service(org_by_name)
        .default_service(web::route().to(HttpResponse::NotFound));
}

#[post("/org")]
async fn org_create(new_org: web::Json<NewOrg>) -> impl Responder {
    let org = dal::org::Org::create(&*new_org);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(org)
}

#[get("/org/list")]
async fn org_read() -> HttpResponse {
    let org = dal::org::Org::read();
    HttpResponse::Ok().json(org)
}

#[put("/org")]
async fn org_update(updated_org: web::Json<UpdatedOrg>) -> impl Responder {
    let org = dal::org::Org::update(&*updated_org);
    HttpResponse::Ok().json(org)
}

#[delete("/org/{id}")]
async fn org_delete(id: web::Path<String>) -> impl Responder {
    println!("Deleting org with id {}", id);
    let id_parse_result = id.parse::<i64>();

    let response = match id_parse_result {
        Ok(i64_id) => {
            let success = dal::org::Org::delete(i64_id);
            let content = format!("{{ success: {} }}", success);
            content
        }
        Err(_e) => {
            let content = format!("{{ error: '{}' }}", "Invalid id");
            content
        }
    };

    HttpResponse::Ok().json(response)
}

#[get("/org/{name}")]
async fn org_by_name(name: web::Path<String>, filter: web::Query<Filter>) -> impl Responder {
    let org = dal::org::Org::get_by_name(&name.to_string());
    println!("{}", filter.org_name);
    HttpResponse::Ok().json(org)
}
