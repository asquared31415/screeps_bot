//! code responsible for managing a single "colony"
//!
//! a "colony" is a single unit that manages a collection of one or more rooms. it has complete
//! control over those rooms, while other colonies might send a creep through a room it does not
//! control, they will never interfere with actions.
pub mod inventory;
pub mod memory;

pub use self::inventory::{
    Inventory, Reservation, ReservationError, ReservationId, Target, TargetInfo,
};

mod room;

use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::{colony::room::RoomInfo, state::GlobalState};

#[derive(Debug, Clone, Copy)]
pub struct ColonyId(u32);

impl ColonyId {
    fn next(next_colony_id: &mut u32) -> Self {
        let id = *next_colony_id;
        *next_colony_id = next_colony_id.checked_add(1).expect("u32::MAX colony IDs");
        Self(id)
    }
}

#[derive(Debug)]
pub struct Colony {
    name: String,
    rooms: Vec<RoomInfo>,
}

impl Colony {
    pub fn new(state: &mut GlobalState, base_room: RoomName) -> Self {
        Self {
            name: String::from("uwu"),
            rooms: todo!(),
        }
    }
}
