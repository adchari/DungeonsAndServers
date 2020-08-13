#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;

type PlayerMap = Mutex<HashMap<String, Player>>;

#[derive(Serialize, Deserialize, Clone)]
struct Player {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    err: Option<String>,
    token: Option<String>,
}

#[post("/players", format = "json", data = "<message>")]
fn new_player(message: Json<Player>, map: State<PlayerMap>) -> Json<Response> {
    let mut hashmap = map.lock().unwrap();
    let id = Uuid::new_v4().to_hyphenated().to_string();
    hashmap.insert(id.clone(), message.0);
    Json(Response {
        err: None,
        token: Some(id),
    })
}

#[get("/players", format = "json")]
fn get_all_players(map: State<PlayerMap>) -> Json<HashMap<String, Player>> {
    let hashmap = &*(map.lock().unwrap());
    Json(hashmap.clone())
}

#[get("/players/<id>", format = "json")]
fn get_player(id: String, map: State<PlayerMap>) -> Option<Json<Player>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[catch(404)]
fn not_found() -> Json<Response> {
    Json(Response {
        err: Some("Could not find requested resource".to_string()),
        token: None,
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_player, new_player, get_all_players])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<String, Player>::new()))
        .launch();
}
