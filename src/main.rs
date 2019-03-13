#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate chrono;

use rocket_contrib::databases::diesel::SqliteConnection;
use diesel::prelude::*;
use schema::*;
use models::*;

use rocket_contrib::templates::Template;
use serde::Serialize;
use chrono::prelude::Local;

pub mod schema;
pub mod models;


#[database("db")]
struct PlantDbConn(SqliteConnection);


#[derive(Serialize)]
struct PalmContext {
    data: Vec<models::PalmLogEntry>
}


#[get("/")]
fn index() -> &'static str {
    "Hello, you cheeky bastard"
}

#[get("/palm")]
fn get_soil_moisture(conn: PlantDbConn) -> Template {

    let results = palm_log::table
        .load::<PalmLogEntry>(&*conn)
        .expect("Error loading posts");

    Template::render("index", PalmContext { data: results })
}

#[put("/palm/<moisture>")]
fn log_soil_moisture(conn: PlantDbConn, moisture: i32) {

    let new_entry = PalmLogEntry {
        log_time: Local::now().to_rfc3339(),
        moisture: moisture
    };

    diesel::insert_into(palm_log::table)
        .values(new_entry)
        .execute(&*conn)
        .expect("Error inserting into database");
}



fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_soil_moisture, log_soil_moisture])
        .attach(Template::fairing())
        .attach(PlantDbConn::fairing())
        .launch();
}
