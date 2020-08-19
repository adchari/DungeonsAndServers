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

pub mod types;
use crate::types::{
    Game, GameMap, ItemManual, ItemManualMap, MonsterManual, MonsterManualMap, NPCManual,
    NPCManualMap, Player, PlayerMap, Response, World, WorldMap,
};

// Game
#[get("/game/create/<world>/<items>/<monsters>/<npcs>")]
fn create_game(
    world: String,
    items: String,
    monsters: String,
    npcs: String,
    game_map: State<GameMap>,
    world_map: State<WorldMap>,
    item_map: State<ItemManualMap>,
    monster_map: State<MonsterManualMap>,
    npc_map: State<NPCManualMap>,
) -> Option<Json<Response>> {
    let locked_world = world_map.lock().unwrap();
    let locked_item = item_map.lock().unwrap();
    let locked_monster = monster_map.lock().unwrap();
    let locked_npc = npc_map.lock().unwrap();

    match (
        locked_world.get(&world),
        locked_item.get(&items),
        locked_monster.get(&monsters),
        locked_npc.get(&npcs),
    ) {
        (Some(w), Some(i), Some(m), Some(n)) => {
            let mut hashmap = game_map.lock().unwrap();
            let id = Uuid::new_v4().to_hyphenated().to_string();
            hashmap.insert(
                id.clone(),
                Game {
                    world: w.clone(),
                    items: i.clone(),
                    npcs: n.clone(),
                    monsters: m.clone(),
                    players: HashMap::<String, String>::new(),
                },
            );
            Some(Json(Response {
                err: None,
                token: Some(id),
            }))
        }
        _ => None,
    }
}

// Players
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

#[get("/players")]
fn get_all_players(map: State<PlayerMap>) -> Json<Vec<String>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.keys().cloned().collect::<Vec<String>>())
}

#[get("/players/<id>")]
fn get_player(id: String, map: State<PlayerMap>) -> Option<Json<Player>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[delete("/players/<id>")]
fn remove_player(id: String, map: State<PlayerMap>) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    hashmap.remove(&id).map(|_key| {
        Json(Response {
            err: None,
            token: Some(id),
        })
    })
}

// Worlds
#[post("/worlds", format = "json", data = "<message>")]
fn new_world(message: Json<World>, map: State<WorldMap>) -> Json<Response> {
    let mut hashmap = map.lock().unwrap();
    let id = Uuid::new_v4().to_hyphenated().to_string();
    hashmap.insert(id.clone(), message.0);
    Json(Response {
        err: None,
        token: Some(id),
    })
}

#[put("/worlds/<id>", format = "json", data = "<message>")]
fn update_world(id: String, message: Json<World>, map: State<WorldMap>) -> Option<Json<Response>> {
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

#[get("/worlds")]
fn get_all_worlds(map: State<WorldMap>) -> Json<Vec<String>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.keys().cloned().collect::<Vec<String>>())
}

#[get("/worlds/<id>")]
fn get_world(id: String, map: State<WorldMap>) -> Option<Json<World>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[delete("/worlds/<id>")]
fn remove_world(id: String, map: State<WorldMap>) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    hashmap.remove(&id).map(|_key| {
        Json(Response {
            err: None,
            token: Some(id),
        })
    })
}

// NPCs
#[post("/npcs", format = "json", data = "<message>")]
fn new_npc(message: Json<NPCManual>, map: State<NPCManualMap>) -> Json<Response> {
    let mut hashmap = map.lock().unwrap();
    let id = Uuid::new_v4().to_hyphenated().to_string();
    hashmap.insert(id.clone(), message.0);
    Json(Response {
        err: None,
        token: Some(id),
    })
}

#[put("/npcs/<id>", format = "json", data = "<message>")]
fn update_npc(
    id: String,
    message: Json<NPCManual>,
    map: State<NPCManualMap>,
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

#[get("/npcs")]
fn get_all_npcs(map: State<NPCManualMap>) -> Json<Vec<String>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.keys().cloned().collect::<Vec<String>>())
}

#[get("/npcs/<id>")]
fn get_npc(id: String, map: State<NPCManualMap>) -> Option<Json<NPCManual>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[delete("/npcs/<id>")]
fn remove_npc(id: String, map: State<NPCManualMap>) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    hashmap.remove(&id).map(|_key| {
        Json(Response {
            err: None,
            token: Some(id),
        })
    })
}

// Monsters
#[post("/monsters", format = "json", data = "<message>")]
fn new_monster(message: Json<MonsterManual>, map: State<MonsterManualMap>) -> Json<Response> {
    let mut hashmap = map.lock().unwrap();
    let id = Uuid::new_v4().to_hyphenated().to_string();
    hashmap.insert(id.clone(), message.0);
    Json(Response {
        err: None,
        token: Some(id),
    })
}

#[put("/monsters/<id>", format = "json", data = "<message>")]
fn update_monster(
    id: String,
    message: Json<MonsterManual>,
    map: State<MonsterManualMap>,
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

#[get("/monsters")]
fn get_all_monsters(map: State<MonsterManualMap>) -> Json<Vec<String>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.keys().cloned().collect::<Vec<String>>())
}

#[get("/monsters/<id>")]
fn get_monster(id: String, map: State<MonsterManualMap>) -> Option<Json<MonsterManual>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[delete("/monsters/<id>")]
fn remove_monster(id: String, map: State<MonsterManualMap>) -> Option<Json<Response>> {
    let mut hashmap = map.lock().unwrap();
    hashmap.remove(&id).map(|_key| {
        Json(Response {
            err: None,
            token: Some(id),
        })
    })
}

// Items
#[post("/items", format = "json", data = "<message>")]
fn new_item(message: Json<ItemManual>, map: State<ItemManualMap>) -> Json<Response> {
    let mut hashmap = map.lock().unwrap();
    let id = Uuid::new_v4().to_hyphenated().to_string();
    hashmap.insert(id.clone(), message.0);
    Json(Response {
        err: None,
        token: Some(id),
    })
}

#[put("/items/<id>", format = "json", data = "<message>")]
fn update_item(
    id: String,
    message: Json<ItemManual>,
    map: State<ItemManualMap>,
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

#[get("/items")]
fn get_all_items(map: State<ItemManualMap>) -> Json<Vec<String>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.keys().cloned().collect::<Vec<String>>())
}

#[get("/items/<id>")]
fn get_item(id: String, map: State<ItemManualMap>) -> Option<Json<ItemManual>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| Json(contents.clone()))
}

#[delete("/items/<id>")]
fn remove_item(id: String, map: State<ItemManualMap>) -> Option<Json<Response>> {
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
                remove_player,
                get_world,
                new_world,
                get_all_worlds,
                update_world,
                remove_world,
                get_npc,
                new_npc,
                get_all_npcs,
                update_npc,
                remove_npc,
                get_monster,
                new_monster,
                get_all_monsters,
                update_monster,
                remove_monster,
                get_item,
                new_item,
                get_all_items,
                update_item,
                remove_item,
                create_game,
            ],
        )
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<String, Player>::new()))
        .manage(Mutex::new(HashMap::<String, World>::new()))
        .manage(Mutex::new(HashMap::<String, MonsterManual>::new()))
        .manage(Mutex::new(HashMap::<String, NPCManual>::new()))
        .manage(Mutex::new(HashMap::<String, ItemManual>::new()))
        .manage(Mutex::new(HashMap::<String, Game>::new()))
        .launch();
}
