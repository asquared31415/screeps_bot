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
pub struct Memory {
    pub creeps: HashMap<String, CreepMemory>,
}

#[derive(Debug)]
pub struct CreepMemory {
    pub task: TaskState,
}

#[derive(Debug)]
pub enum TaskState {
    Haul(HaulState),
}

#[derive(Debug)]
pub enum HaulState {
    Gathering,
    Delivering,
}
