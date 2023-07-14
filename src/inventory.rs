use screeps::{ObjectId, Resource, ResourceType, StructureStorage};
use std::collections::HashMap;

pub struct Inventory {
    // could probably keep a structure that holds totals as an optimization
    reservations: HashMap<Target, Vec<Reservation>>,
}

impl Inventory {
    pub fn reserve(
        &mut self,
        target: impl Into<Target>,
        kind: ResourceType,
        reserve_amount: u32,
    ) -> Result<Reservation, ReservationError> {
        let target = target.into();
        let existing_reservations = self.reservations.entry(target).or_default();
        match target {
            Target::Resource(id) => {
                if let Some(resource) = id.resolve() {
                    let total_amount = resource.amount();
                    let already_reserved = existing_reservations
                        .iter()
                        .fold(0, |acc, x| acc + x.amount);

                    let available = total_amount - already_reserved;

                    // TODO: maybe not let less than X amount be reserved if it's already much smaller than the request

                    // take as many as requested, or however many is available, whichever is fewer
                    let final_amount = u32::min(reserve_amount, available);
                    Ok(Reservation {
                        target,
                        kind,
                        amount: final_amount,
                    })
                } else {
                    Err(ReservationError::NotFound)
                }
            }
            Target::Storage(id) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Reservation {
    target: Target,
    kind: ResourceType,
    amount: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ReservationError {
    /// The specified target could not be found, possibly due to vision or it not existing.
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Resource(ObjectId<Resource>),
    Storage(ObjectId<StructureStorage>),
    //...
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
