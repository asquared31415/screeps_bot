use log::warn;
use screeps::{Creep, ErrorCode, HasPosition, ObjectId, SharedCreepProperties, Source};

use crate::tasks::TaskResult;

pub fn run(source_id: &ObjectId<Source>, creep: &Creep) -> TaskResult {
    let creep_pos = creep.pos();
    let Some(source) = source_id.resolve() else {
        warn!("source id {} didn't resolve", source_id);
        return TaskResult::Error;
    };
    if creep_pos.is_near_to(source.pos()) {
        match creep.harvest(&source) {
            Ok(()) | Err(ErrorCode::NotEnough) => TaskResult::InProgress,
            Err(e) => {
                warn!(
                    "creep {} unexpected error {:?} when harvesting",
                    creep.name(),
                    e
                );
                TaskResult::Error
            }
        }
    } else {
        let _ = creep.move_to(&source);
        TaskResult::InProgress
    }
}
