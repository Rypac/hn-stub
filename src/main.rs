#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use rocket_contrib::{Json, Value};

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Json<Value>, Box<Error>> {
    let file = File::open(path)?;
    let json = serde_json::from_reader(file)?;
    Ok(Json(json))
}

mod routes {
    use std::collections::HashMap;
    use rocket::State;
    use rocket_contrib::{Json, Value};

    type Responses = HashMap<String, Json<Value>>;

    #[get("/firebase/<resource>")]
    fn firebase(resource: String, map: State<Responses>) -> Option<Json<Value>> {
        map.get("firebase")
            .map(|data| Json(data[&resource].clone()))
    }

    #[get("/algolia/<resource>")]
    fn algolia(resource: String, map: State<Responses>) -> Option<Json<Value>> {
        map.get("algolia").map(|data| Json(data[&resource].clone()))
    }
}

fn main() {
    let mut data = HashMap::<String, Json<Value>>::new();
    let firebase = read_json_from_file("./data/firebase.json").expect("Firebase data should exist");
    let algolia = read_json_from_file("./data/algolia.json").expect("Algolia data should exist");
    data.insert("firebase".to_owned(), firebase);
    data.insert("algolia".to_owned(), algolia);
    rocket::ignite()
        .mount("/", routes![routes::firebase, routes::algolia])
        .manage(data)
        .launch();
}
