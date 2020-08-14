use std::collections::HashMap;
use std::sync::Mutex;

pub type PlayerMap = Mutex<HashMap<String, Player>>;
pub type WorldMap = Mutex<HashMap<String, World>>;
pub type MonsterManualMap = Mutex<HashMap<String, MonsterManual>>;
pub type NPCManualMap = Mutex<HashMap<String, NPCManual>>;
pub type ItemManualMap = Mutex<HashMap<String, ItemManual>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterManual {
    pub monsters: Vec<Monster>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPCManual {
    pub npcs: Vec<NPC>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPC {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemManual {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub err: Option<String>,
    pub token: Option<String>,
}
