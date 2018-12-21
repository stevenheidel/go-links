#![allow(proc_macro_derive_resolution_fallback)] // Silence Diesel warnings that will be fixed in Diesel 1.4
#![feature(decl_macro, proc_macro_hygiene, uniform_paths)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::PathBuf;

use diesel::prelude::*;
use dotenv::dotenv;

use schema::links::dsl::*;

mod schema;

#[database("production")]
struct DbConn(PgConnection);

#[derive(Debug, Queryable)]
struct Link {
    pub id: i32,
    pub src: String,
    pub dest: String,
    pub hits: i32,
}

impl Link {
    fn all(conn: &PgConnection) -> QueryResult<Vec<Link>> {
        links.load(&conn)
    }
}

#[get("/<path..>")]
fn index(conn: DbConn, path: PathBuf) -> String {
    format!("{:?}", Link::all(&conn))
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .launch();
}
