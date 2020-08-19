use std::collections::HashMap;
use std::sync::Mutex;

pub type PlayerMap = Mutex<HashMap<String, Player>>;
pub type WorldMap = Mutex<HashMap<String, World>>;
pub type MonsterManualMap = Mutex<HashMap<String, MonsterManual>>;
pub type NPCManualMap = Mutex<HashMap<String, NPCManual>>;
pub type ItemManualMap = Mutex<HashMap<String, ItemManual>>;
pub type GameMap = Mutex<HashMap<String, Game>>;

#[derive(Serialize, Deserialize, Clone)]
pub enum Stat {
    STR,
    DEX,
    CON,
    INT,
    WIS,
    CHA,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TunnelStatus {
    OPEN,
    LOCKED,
    GUARDED,
}

pub struct Game {
    pub world: World,                     // The actual world (must be cloned)
    pub items: ItemManual,                // Reference to an ItemManual
    pub npcs: NPCManual,                  // Reference to an NPCManual
    pub monsters: MonsterManual,          // Reference to a MonsterManual
    pub players: HashMap<String, String>, // Maps player strings to their locations
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Player {
    pub name: String,
    pub ac: u8,
    pub level: u8,
    pub xp: u16,
    pub hp: u16,
    pub STR: u8,
    pub DEX: u8,
    pub CON: u8,
    pub INT: u8,
    pub WIS: u8,
    pub CHA: u8,
    pub inventory: Vec<String>,
    pub attacks: Vec<Attack>,
}

/// An attack consists of a primary roll to assess whether the target was hit.
/// This roll is 1d20 + modifier for the related skill (e.g. a sword attack may use STR, while a bow may use DEX)
/// The second roll is the damage, which is represented using a string like "1d5+7"
#[derive(Serialize, Deserialize, Clone)]
pub struct Attack {
    pub name: String,
    pub stat: Stat,
    pub damage: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub start: String,
    pub locations: HashMap<String, Location>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub description: HashMap<u8, String>,
    pub paths: Vec<Path>,
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub monsters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Path {
    pub endpoint: String,
    pub status: TunnelStatus,
    pub key: Option<String>,
    pub guard: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterManual {
    pub monsters: HashMap<String, Monster>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPCManual {
    pub npcs: HashMap<String, NPC>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPC {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemManual {
    pub items: HashMap<String, Item>,
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
