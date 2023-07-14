use screeps::{ObjectId, Position, Source};

#[derive(Debug)]
pub enum Task {
    DropHarvest(ObjectId<Source>, Position),
    HaulResource(()),
}
