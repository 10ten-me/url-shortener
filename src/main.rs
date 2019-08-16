#![feature(proc_macro_hygiene, decl_macro, custom_attribute, type_alias_enum_variants)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate serde;
extern crate rocket_contrib;
extern crate chrono;
extern crate nanoid;

pub mod msg;
pub mod middleware;
pub mod schema;
pub mod model;
pub mod generator;
mod shorten;
mod unshorten;

static NOT_FOUND: &str = "/not-found";
static DATABASE_URL: &str = "PG_URL";

#[get("/")]
fn not_found() -> String {
    format!("NOT FOUND")
}

fn main() {
    rocket::ignite()
        .manage(middleware::init_db_pool(std::env::var(DATABASE_URL).unwrap()))
        .mount("/shorten", routes![shorten::build_shortened])
        .mount("/", routes![unshorten::redirect_to_initial])
        .mount("/not-found", routes![not_found])
        .launch();
}
