use log::warn;
use screeps::{Creep, ErrorCode, HasPosition, ObjectId, SharedCreepProperties, Source};

pub fn run(source_id: &ObjectId<Source>, creep: &Creep) {
    let creep_pos = creep.pos();
    let Some(source) = source_id.resolve() else {
        warn!("source id {} didn't resolve", source_id);
        return;
    };
    if creep_pos.is_near_to(source.pos()) {
        match creep.harvest(&source) {
            Ok(()) | Err(ErrorCode::NotEnough) => {}
            Err(e) => {
                warn!(
                    "creep {} unexpected error {:?} when harvesting",
                    creep.name(),
                    e
                );
            }
        }
    } else {
        let _ = creep.move_to(&source);
    }
}
