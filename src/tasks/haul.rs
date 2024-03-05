use log::*;
use screeps::{
    find, game, prelude::*, Creep, ErrorCode, RawObjectId, Room, StoreObject, StructureObject,
    StructureType,
};
use wasm_bindgen::JsValue;

use crate::{
    colony::{Inventory, ReservationId, Target},
    state::HaulState,
    tasks::TaskResult,
};

pub fn find_target(inventory: &mut Inventory, room: &Room) -> Option<(ReservationId, RawObjectId)> {
    let structures = room.find(find::MY_STRUCTURES, None);
    let mut structures = structures
        .into_iter()
        .filter_map(|s| {
            let store_object = StoreObject::try_from(s.clone()).ok()?;
            let resource_types = store_object.resource_types();

            resource_types
                .iter()
                .any(|&ty| store_object.store().get_free_capacity(Some(ty)) > 0)
                .then_some((s, resource_types))
        })
        .collect::<Vec<_>>();
    structures.sort_by_key(|s| TargetSortOrder::from(&s.0));

    if let Some((target, resource_types)) = structures.first() {
        let amount = 50;
        let Some(reservation_id) = resource_types
            .iter()
            .find_map(|&kind| inventory.request(kind, amount).ok())
        else {
            debug!(
                "unable to reserve resources for {}",
                target.structure_type()
            );
            return None;
        };

        debug!("got reservation {:?}", reservation_id);

        let id = target.as_structure().raw_id();
        Some((reservation_id, id))
    } else {
        warn!("no haul target found");
        None
    }
}

pub fn run(
    state: &mut HaulState,
    inventory: &mut Inventory,
    creep: &Creep,
    reservation_id: &ReservationId,
    target: &RawObjectId,
) -> TaskResult {
    let Some(reservation) = inventory.resolve_reservation(reservation_id) else {
        return TaskResult::Error;
    };

    match state {
        HaulState::Gathering => match reservation.target() {
            Target::Resource(id) => {
                // resolving a reservation ensures that the target exists
                let resource = id.resolve().unwrap();
                if creep.pos().is_near_to(resource.pos()) {
                    match creep.pickup(&resource) {
                        Ok(()) | Err(ErrorCode::Full) => {
                            *state = HaulState::Delivering;
                            TaskResult::InProgress
                        }
                        Err(ErrorCode::InvalidTarget) => {
                            warn!("target not valid");
                            inventory.release(*reservation_id);
                            TaskResult::Error
                        }
                        Err(e) => {
                            warn!("unexpected error {:?}", e);
                            inventory.release(*reservation_id);
                            TaskResult::Error
                        }
                    }
                } else {
                    let _ = creep.move_to(resource);
                    TaskResult::InProgress
                }
            }
            Target::Storage(id) => todo!(),
        },
        HaulState::Delivering => {
            let Some(target) = game::get_object_by_id_erased(target) else {
                warn!("creep {} could no longer find {}", creep.name(), target);
                inventory.release(*reservation_id);
                return TaskResult::Error;
            };
            let structure = StructureObject::from(JsValue::from(target));
            let Some(transferrable) = structure.as_transferable() else {
                warn!("structure was not transferrable");
                inventory.release(*reservation_id);
                return TaskResult::Error;
            };

            if creep.pos().is_near_to(structure.pos()) {
                match creep.transfer(transferrable, reservation.resource_type(), None) {
                    Ok(()) => {
                        inventory.release(*reservation_id);
                        TaskResult::Complete
                    }
                    Err(ErrorCode::Full) => {
                        // TODO: handle this by returning resources somewhere maybe?
                        inventory.release(*reservation_id);
                        TaskResult::Error
                    }
                    Err(e) => {
                        warn!("unexpected error {:?}", e);
                        inventory.release(*reservation_id);
                        TaskResult::Error
                    }
                }
                // TODO: if empty, exit
            } else {
                let _ = creep.move_to(structure);
                TaskResult::InProgress
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// a helper to allow easier sorting of structures based on what should be transferred to first.
enum TargetSortOrder {
    Spawn,
    Extension,
    Tower,
    Storage,
    Other,
}

impl From<&StructureObject> for TargetSortOrder {
    fn from(value: &StructureObject) -> Self {
        match value.structure_type() {
            StructureType::Spawn => TargetSortOrder::Spawn,
            StructureType::Extension => TargetSortOrder::Extension,
            StructureType::Tower => TargetSortOrder::Tower,
            StructureType::Storage => TargetSortOrder::Storage,
            _ => TargetSortOrder::Other,
        }
    }
}
