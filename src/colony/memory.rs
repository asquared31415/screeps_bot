use screeps::RoomName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColonyMemory {
    name: String,
    rooms: Vec<RoomName>,
}
