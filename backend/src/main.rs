/**
 * \file main.rs
 *
 * \brief Main file of the backend. It instantiates the server and creates the
 *        different endpoints.
 *
 * \author Mathieu Dique
 */
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
async fn get_regions(region_query: web::Query<query::Region>) -> impl Responder {
    let regions: Vec<entity::Region> = parse::regions(&region_query.url()).await;

    let regions_serialized: String = serde_json::to_string(&regions).unwrap();

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(regions_serialized)
}

/*
 * Departments endpoint, requires the region url department url and the region
 * name.
 */
#[get("/departments")]
async fn get_departments(department_query: web::Query<query::Department>) -> impl Responder {
    let departments: Vec<entity::Department> =
        parse::departments(department_query.url(), department_query.region()).await;

    let departments_serialized: String = serde_json::to_string(&departments).unwrap();

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(departments_serialized)
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
            .service(get_departments)
            .default_service(web::to(|| handle_not_found()))
    };
    HttpServer::new(application)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
