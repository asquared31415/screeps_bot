use core::sync::atomic::{self, AtomicU32};
use std::collections::{HashMap, HashSet};

use log::warn;
use screeps::{find, HasId, ObjectId, Resource, ResourceType, Room, StructureStorage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct RoomInventory {
    /// all possible known targets that could serve a request
    targets: HashMap<Target, TargetInfo>,
    /// all the reservations that this inventory knows about
    reservations: HashMap<ReservationId, Reservation>,
}

impl RoomInventory {
    pub fn request(
        &mut self,
        kind: ResourceType,
        amount: u32,
    ) -> Result<ReservationId, ReservationError> {
        // find a suitable target
        let target: Option<(&Target, &mut TargetInfo)> =
            self.targets.iter_mut().find(|(target, info)| {
                // TODO: make multiple reservations per target allowed
                if info.reservations.len() > 0 {
                    return false;
                }

                target
                    .get_available_amount(kind)
                    .is_some_and(|avl| avl > amount)
            });

        if let Some((&target, info)) = target {
            let id = ReservationId::next();
            info.reservations.insert(id);

            let reservation = Reservation {
                target,
                kind,
                amount,
            };
            self.reservations.insert(id, reservation);

            Ok(id)
        } else {
            Err(ReservationError::NotEnough(NotEnoughErr { kind, amount }))
        }
    }

    pub fn release(&mut self, id: ReservationId) {
        let Some(Reservation { target, .. }) = self.reservations.get(&id) else {
            warn!("reservation {:?} did not exist", id);
            return;
        };
        let Some(target_info) = self.targets.get_mut(&target) else {
            warn!("reservation {:?} had invalid target", id);
            return;
        };

        target_info.reservations.remove(&id);
        self.reservations.remove(&id);
    }

    pub fn update_targets(&mut self, room: &Room) {
        // TODO: scan more than resources
        let mut seen = HashSet::<Target>::new();
        for resource in room.find(find::DROPPED_RESOURCES, None) {
            let target = Target::from(resource.id());
            seen.insert(target);

            // insert targets that don't yet exist
            if !self.targets.contains_key(&target) {
                self.targets.insert(
                    target,
                    TargetInfo {
                        reservations: HashSet::new(),
                    },
                );
            }
        }

        // clear out targets that no longer exist
        self.targets.retain(|target, _| seen.contains(target));
    }

    // TODO: differentiate between "no reservation" and "no target?" in a result?
    pub fn resolve_reservation(&self, id: &ReservationId) -> Option<&Reservation> {
        let reservation = self.reservations.get(id)?;
        if !self.targets.contains_key(&reservation.target) {
            warn!("reservation {:?} no longer has a target", id);
            return None;
        }

        Some(reservation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReservationId(u32);

impl ReservationId {
    /// gets the next reservation id and increments the id counter
    fn next() -> Self {
        /// the next id to use for a reservation
        static NEXT_RESERVATION_ID: AtomicU32 = AtomicU32::new(0);

        let val = NEXT_RESERVATION_ID.fetch_add(1, atomic::Ordering::Relaxed);
        Self(val)
    }
}

/// describes the current state of a single target, namely all active reservations
#[derive(Debug)]
pub struct TargetInfo {
    reservations: HashSet<ReservationId>,
}

#[derive(Debug)]
pub struct Reservation {
    target: Target,
    kind: ResourceType,
    amount: u32,
}

impl Reservation {
    pub fn target(&self) -> Target {
        self.target
    }

    pub fn resource_type(&self) -> ResourceType {
        self.kind
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

#[derive(Debug, Clone, Copy)]
/// describes the way in which a request for resources failed
pub enum ReservationError {
    /// the request for a resource could not be filled because there was not enough of this
    /// resource in any tracked target
    NotEnough(NotEnoughErr),
}

#[derive(Debug, Clone, Copy)]
pub struct NotEnoughErr {
    pub kind: ResourceType,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Resource(ObjectId<Resource>),
    Storage(ObjectId<StructureStorage>),
    // TODO: containers (but make sure to only add source ones)
}

impl Target {
    /// gets the available abount of resource `kind` in this target, if the target can be found
    ///
    /// **WARNING** this does not account for reservations, this is the **total** amount
    pub fn get_available_amount(&self, kind: ResourceType) -> Option<u32> {
        match self {
            Target::Resource(id) => id.resolve().map(|resource| {
                if resource.resource_type() == kind {
                    resource.amount()
                } else {
                    0
                }
            }),
            Target::Storage(id) => id
                .resolve()
                .map(|storage| storage.store().get_used_capacity(Some(kind))),
        }
    }
}

impl From<ObjectId<Resource>> for Target {
    fn from(value: ObjectId<Resource>) -> Self {
        Self::Resource(value)
    }
}

impl From<ObjectId<StructureStorage>> for Target {
    fn from(value: ObjectId<StructureStorage>) -> Self {
        Self::Storage(value)
    }
}
