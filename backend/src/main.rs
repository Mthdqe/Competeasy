/* -------------------------------------------------------------------------- */
pub mod parse;
pub mod scrap;
pub mod utils;

/* -------------------------------------------------------------------------- */
use crate::utils::*;

/* -------------------------------------------------------------------------- */
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

/* -------------------------------------------------------------------------- */
/*
 * Competitions endpoint, do not take any argument as competitions are hard
 * coded
 */
#[get("/competitions")]
async fn get_competitions() -> impl Responder {
    let competitions: Vec<entity::Competition> = parse::competitions();

    let competitions_serialized: String = serde_json::to_string(&competitions).unwrap();

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(competitions_serialized)
}

/*
 * Regions endpoint, requires the region url
 */
#[get("/regions")]
async fn get_regions(url: web::Query<entity::Url>) -> impl Responder {
    let regions: Vec<entity::Region> = parse::regions(&url.url()).await;

    let regions_serialized: String = serde_json::to_string(&regions).unwrap();

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(regions_serialized)
}

/* Default response for 404 errors */
async fn handle_not_found() -> impl Responder {
    HttpResponse::NotFound()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body("")
}

/* Main: Creates a web server exposing endpoints */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application = || {
        App::new()
            .service(get_competitions)
            .service(get_regions)
            .default_service(web::to(|| handle_not_found()))
    };
    HttpServer::new(application)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
