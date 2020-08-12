#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json};

type ID = usize;
type PlayerMap = Mutex<HashMap<ID, Player>>;

#[derive(Serialize, Deserialize)]
struct Player {
	id: Option<ID>,
	name: String
}

#[derive(Serialize, Deserialize)]
struct Response {
	err: Option<String>,
	token: Option<ID>
}

#[post("/players", format = "json", data = "<message>")]
fn new_player(message: Json<Player>, map: State<PlayerMap>) -> Json<Response> {
    let mut hashmap = map.lock().expect("map lock.");
	let id: usize = 1;
    hashmap.insert(id, Player {
		id: Some(id),
		name: message.0.name
	});
	Json(Response {
		err: None,
		token: Some(id)
	})
}

#[get("/players/<id>",  format = "json")]
fn get_player(id: ID, map: State<PlayerMap>) -> Option<Json<Player>> {
    let hashmap = map.lock().unwrap();
	hashmap.get(&id).map(|contents| {
		Json(Player {
			id: Some(id),
			name: contents.name.clone()
		})
	})
}

#[catch(404)]
fn not_found() -> Json<Response> {
	Json(Response {
		err: Some("Could not find requested resource".to_string()),
		token: None
	})
}

fn main() {
    rocket::ignite()
		.mount("/", routes![get_player, new_player])
		.register(catchers![not_found])
		.manage(Mutex::new(HashMap::<ID, Player>::new()))
		.launch();
}
