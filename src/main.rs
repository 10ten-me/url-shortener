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

static DATABASE_URL: &str = "PG_URL";

fn main() {
    rocket::ignite()
        .manage(middleware::init_db_pool(std::env::var(DATABASE_URL).unwrap()))
        .mount("/shorten", routes![shorten::build_shortened])
        .mount("/", routes![unshorten::redirect_to_initial])
        .launch();
}
