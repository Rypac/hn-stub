#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use rocket_contrib::{Json, Value};

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Json<Value>, Box<Error>> {
    let file = File::open(path)?;
    let json = serde_json::from_reader(file)?;
    Ok(Json(json))
}

fn read_resource_from_file<P: AsRef<Path>>(path: P, resource: &str) -> Option<Json<Value>> {
    read_json_from_file(path)
        .ok()
        .and_then(|d| d.get(resource).map(Clone::clone))
        .map(Json)
}

mod routes {
    use rocket_contrib::{Json, Value};

    #[get("/firebase/<resource>")]
    fn firebase(resource: String) -> Option<Json<Value>> {
        ::read_resource_from_file("./data/firebase.json", &resource)
    }

    #[get("/algolia/<resource>")]
    fn algolia(resource: String) -> Option<Json<Value>> {
        ::read_resource_from_file("./data/algolia.json", &resource)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::firebase, routes::algolia])
        .launch();
}
