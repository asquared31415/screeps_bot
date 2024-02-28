use std::collections::HashMap;

use screeps::RoomName;

use crate::{inventory::RoomInventory, tasks::RoomTasks};

#[derive(Debug, Default)]
pub struct GlobalState {
    pub room_state: HashMap<RoomName, RoomState>,
    pub memory: Memory,
}

#[derive(Debug, Default)]
pub struct RoomState {
    pub inventory: RoomInventory,
    pub tasks: RoomTasks,
}

#[derive(Debug, Default)]
pub struct Memory {}

#[derive(Debug)]
pub enum HaulState {
    Gathering,
    Delivering,
}
