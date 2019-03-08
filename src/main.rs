#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rusqlite;

use rocket_contrib::templates::Template;
use serde::Serialize;
use rusqlite::{Connection, NO_PARAMS};

const PATH: &str = "plant_data.db";


#[derive(Serialize)]
struct PalmContext {
    data: Vec<MoistureEntry>
}

#[derive(Serialize)]
struct MoistureEntry {  
    date_time: String,
    humidity: u8  
}


#[get("/")]
fn index() -> &'static str {
    "Hello, you cheeky bastard"
}

#[get("/palm")]
fn get_soil_moisture() -> Template {
    let conn = Connection::open(PATH).unwrap();
    let mut statement = conn.prepare("SELECT * FROM palm_humidity limit 72").unwrap();

    let rows = statement
        .query_map(NO_PARAMS, |row| MoistureEntry {
            date_time: row.get(0),
            humidity: row.get(1),
        })
        .unwrap();
    
    let mut palm_vec: Vec<MoistureEntry> = Vec::new();
    for row in rows {
        let entry = row.unwrap();
        let moist_entry = MoistureEntry {
            date_time: entry.date_time, 
            humidity: entry.humidity,
        };
        palm_vec.push(moist_entry);    
    }

    let context = PalmContext { data: palm_vec};
    Template::render("index", context)
}

#[get("/palm/<date_time>/<moisture>")]
fn log_soil_moisture(date_time: String, moisture: u8) {
    let conn = Connection::open(PATH).unwrap();
    conn.execute(
        "INSERT INTO palm_humidity VALUES (?1, ?2);",
        &[&date_time, &moisture.to_string()],
    ).unwrap();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_soil_moisture, log_soil_moisture])
        .attach(Template::fairing())
        .launch();
}