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

#[put("/players/<id>", format = "json", data = "<message>")]
fn update_player(
    id: String,
    message: Json<Player>,
    map: State<PlayerMap>,
) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    if !hashmap.contains_key(&id) {
        return None;
    }

    hashmap.insert(id.clone(), message.0);
    Some(Json(Response {
        err: None,
        token: Some(id),
    }))
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

#[delete("/players/<id>", format = "json")]
fn remove_player(id: String, map: State<PlayerMap>) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    hashmap.remove(&id).map(|_key| {
        Json(Response {
            err: None,
            token: Some(id),
        })
    })
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
        .mount(
            "/",
            routes![
                get_player,
                new_player,
                get_all_players,
                update_player,
                remove_player
            ],
        )
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<String, Player>::new()))
        .launch();
}
