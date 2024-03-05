use core::sync::atomic::AtomicU32;
use std::collections::HashMap;

use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::{colony::Inventory, tasks::RoomTasks};

#[derive(Debug, Default)]
pub struct GlobalState {
    pub room_state: HashMap<RoomName, RoomState>,
    pub memory: Memory,
}

#[derive(Debug, Default)]
pub struct RoomState {
    pub inventory: Inventory,
    pub tasks: RoomTasks,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Memory {
    pub next_colony_id: u32,
}

#[derive(Debug)]
pub enum HaulState {
    Gathering,
    Delivering,
}
