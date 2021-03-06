#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate chrono;

use rocket::request::Form;
use rocket_contrib::databases::diesel::SqliteConnection;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use schema::*;
use models::*;

use rocket_contrib::templates::Template;
use serde::Serialize;
use chrono::prelude::Local;
use std::collections::HashMap;

pub mod schema;
pub mod models;


#[database("db")]
struct DbConn(SqliteConnection);


#[derive(Serialize)]
struct PalmContext {
    data: Vec<models::PalmLogEntry>
}

#[derive(Serialize)]
struct PeerContext {}

#[derive(FromForm)]
struct Name {
    name: String,
}

fn valid_name(name: &str) -> bool {
    match name {
        "Carla" => true,
        "Michael" => true,
        "Lennart" => true,
        _ => false,
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, you cheeky bastard"
}

#[get("/palm")]
fn get_palm_log(conn: DbConn) -> Template  {

    Template::render("index", PalmContext {
        data: palm_log::table
                .load::<PalmLogEntry>(&*conn)
                .expect("Error loading palm_log from database")
        })
}

#[put("/palm/<moisture>")]
fn insert_palm_log_entry(conn: DbConn, moisture: i32) {

    if moisture > 100 || moisture < 0 {
        println!("Cannot log moisture because it is out of bounds");
        return
    }

    let new_entry = PalmLogEntry {
        log_time: Local::now().to_rfc3339(),
        moisture: moisture
    };

    diesel::insert_into(palm_log::table)
        .values(new_entry)
        .execute(&*conn)
        .expect("Error inserting PalmLogEntry into database");
}

#[get("/peer")]
fn peer() -> Template {
    Template::render("peer", PeerContext {})
}

// TODO: if possible, use group by in query
#[get("/peer/standings")]
fn get_standings(conn: DbConn) -> Json<HashMap<String, i32>> {

    let data = achievements::table
    .load::<Achievement>(&*conn)
    .expect("Error loading achievements from database");

    let mut standings: HashMap<String, i32> = HashMap::new();

    for achievement in data {
        let count = standings.entry(achievement.date).or_insert(0);
        *count += 1;
    }

    Json(standings)
}

#[get("/peer/standings/<date>")]
fn get_standings_date(conn: DbConn, date: String) -> Json<Vec<Achievement>> {

    Json(achievements::table
    .filter(achievements::date.eq(&date))
    .load::<Achievement>(&*conn)
    .expect("Error getting results of date from db"))
}

#[post("/peer", data = "<form>")]
fn insert_achievement(conn: DbConn, form: Form<Name>) {

    let name = &form.name;

    if !valid_name(&name) {
        println!("Cannot log achievement because the name is invalid");
        return
    }

    let new_achievement = Achievement {
        name: name.to_string(),
        date: Local::today().to_string()
    };

    // Check that Achievement is not allready logged
    let entries = achievements::table
            .load::<Achievement>(&*conn)
            .expect("Error loading achievements from database");
    for achievement in &entries {
        if achievement.name == new_achievement.name &&
            achievement.date == new_achievement.date {
                println!("Cannot log achievement because it is allready logged");
                return
            }
    }

    diesel::insert_into(achievements::table)
        .values(new_achievement)
        .execute(&*conn)
        .expect("Error inserting new achievement into database");
}


fn main() {
    rocket::ignite()
        .mount("/", routes![
            index, get_palm_log, insert_palm_log_entry,
            peer, get_standings, insert_achievement])
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .launch();
}
