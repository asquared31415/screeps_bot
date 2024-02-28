use log::*;
use screeps::{
    find, game, prelude::HasId as _, Creep, ErrorCode, HasPosition, RawObjectId, Room,
    SharedCreepProperties, StructureObject, StructureProperties, StructureType,
};
use wasm_bindgen::JsValue;

use crate::{
    inventory::{ReservationId, RoomInventory, Target},
    state::HaulState,
    util::StructureExt,
};

pub fn find_target(
    inventory: &mut RoomInventory,
    room: &Room,
) -> Option<(ReservationId, RawObjectId)> {
    let mut structures = room.find(find::MY_STRUCTURES, None);
    structures.retain(|s| s.as_has_store().is_some());
    structures.sort_by_key(|s| TargetSortOrder::from(s));

    if let Some(target) = structures.first() {
        trace!("target: {:?}", target.structure_type());
        let resource_types = target.resource_types();
        let Some(reservation_id) = resource_types
            .iter()
            .find_map(|&kind| inventory.request(kind, 50).ok())
        else {
            warn!("unable to reserve resources for {:?}", target);
            return None;
        };

        let reservation = inventory.resolve_reservation(&reservation_id).unwrap();
        debug!("got reservation {:?}: {:#?}", reservation_id, reservation);

        let id = RawObjectId::from(target.as_structure().id());
        Some((reservation_id, id))
    } else {
        warn!("no haul target found");
        None
    }
}

pub fn run(
    state: &mut HaulState,
    inventory: &mut RoomInventory,
    creep: &Creep,
    reservation_id: &ReservationId,
    target: &RawObjectId,
) {
    let Some(reservation) = inventory.resolve_reservation(reservation_id) else {
        return;
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
                        }
                        Err(ErrorCode::InvalidTarget) => {
                            warn!("target not valid (TODO: remove task)");
                        }
                        Err(e) => warn!("unexpected error {:?}", e),
                    }
                } else {
                    let _ = creep.move_to(resource);
                }
            }
            Target::Storage(id) => todo!(),
        },
        HaulState::Delivering => {
            let Some(target) = game::get_object_by_id_erased(target) else {
                warn!("creep {} could no longer find {}", creep.name(), target);
                return;
            };
            let structure = StructureObject::from(JsValue::from(target));
            let Some(transferrable) = structure.as_transferable() else {
                warn!("structure was not transferrable");
                return;
            };

            if creep.pos().is_near_to(structure.pos()) {
                match creep.transfer(transferrable, reservation.resource_type(), None) {
                    Ok(()) => {
                        // TODO: complete task
                    }
                    Err(ErrorCode::Full) => {
                        // TODO: handle this by returning resources somewhere maybe?
                    }
                    Err(e) => warn!("unexpected error {:?}", e),
                }
                // TODO: if empty, exit
            } else {
                let _ = creep.move_to(structure);
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
