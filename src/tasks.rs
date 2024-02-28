use std::collections::HashMap;

use log::{debug, trace, warn};
use screeps::{
    find, game, Creep, HasId, MaybeHasId, ObjectId, Part, RawObjectId, Room, SharedCreepProperties,
    Source,
};

use crate::{
    inventory::{ReservationId, RoomInventory},
    state::HaulState,
    GlobalState,
};

mod drop_harvest;
mod haul;

#[derive(Debug)]
pub enum Task {
    DropHarvest(ObjectId<Source>),
    /// a task to haul a specified resource to a target store
    /// INVARIANT: the target store must always be able to store the resource type for the
    /// reservation. it may not have room for the reservation, but code will handle that on a
    /// case-by-case basis.
    Haul(HaulState, ReservationId, RawObjectId),
}

impl Task {
    fn execute(&mut self, inventory: &mut RoomInventory, creep: &Creep) -> TaskResult {
        match self {
            Task::DropHarvest(source_id) => drop_harvest::run(source_id, creep),
            Task::Haul(haul_state, reservation_id, target) => {
                haul::run(haul_state, inventory, creep, reservation_id, target)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskResult {
    // the task was completed, remove it from the task list
    Complete,
    // the task is still in progress
    InProgress,
    // the task was not able to complete, but cannot be continued
    Error,
}

#[derive(Debug, Default)]
pub struct RoomTasks {
    tasks: HashMap<ObjectId<Creep>, Task>,
}

pub fn process_tasks(state: &mut GlobalState) {
    for creep in game::creeps().values() {
        debug!("processing creep {}", creep.name());
        if creep.spawning() {
            trace!("skipping spawning creep");
            continue;
        }

        // creeps always have a room
        let room = creep.room().unwrap();
        let room_name = room.name();
        let Some(room_state) = state.room_state.get_mut(&room_name) else {
            warn!("no RoomState for {}", room_name);
            continue;
        };

        let inventory = &mut room_state.inventory;
        let tasks = &mut room_state.tasks;

        let id = creep
            .try_id()
            .expect("creeps that have been spawned should have an id");
        if tasks.tasks.contains_key(&id) {
            execute_task_common(tasks, id, inventory);
        } else {
            debug!("reassigning task for {}", creep.name());
            // reassign task based on what creep would be most suited for
            if let Some(task) = find_best_task(&creep, &room, inventory) {
                debug!("creep {}: {:?}", creep.name(), task);
                tasks.tasks.insert(id, task);
                execute_task_common(tasks, id, inventory);
            } else {
                debug!("creep {} not assigned a task", creep.name());
            }
        }
    }
}

/// INVARIANT: `id` must correspond to a creep that exists and it must have a task in `tasks`
fn execute_task_common(tasks: &mut RoomTasks, id: ObjectId<Creep>, inventory: &mut RoomInventory) {
    let creep = id.resolve().unwrap();
    let task = tasks.tasks.get_mut(&id).unwrap();
    debug!("executing task {:?} for {}", task, creep.name());
    match task.execute(inventory, &creep) {
        TaskResult::Complete | TaskResult::Error => {
            tasks.tasks.remove(&id);
        }
        TaskResult::InProgress => {}
    }
}

fn find_best_task(creep: &Creep, room: &Room, inventory: &mut RoomInventory) -> Option<Task> {
    let mut task = None;
    for body_part in creep.body() {
        let part = body_part.part();
        match part {
            // creeps that can work should be harvesters
            Part::Work => {
                let sources = room.find(find::SOURCES, None);
                let Some(source) = sources.first() else {
                    // maybe the creep can do something else
                    continue;
                };
                task = Some(Task::DropHarvest(source.id()));
            }
            // creeps that can carry should be haulers
            Part::Carry => {
                let Some((reservation, target)) = haul::find_target(inventory, &room) else {
                    break;
                };
                task = Some(Task::Haul(HaulState::Gathering, reservation, target));
            }
            _ => {}
        }
    }
    task
}
