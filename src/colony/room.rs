use screeps::RoomName;

#[derive(Debug)]
pub struct RoomInfo {
    name: RoomName,
    kind: RoomKind,
}

#[derive(Debug)]
pub enum RoomKind {
    /// an owned room that is meant to be the "center" of a colony, will have spawns and other
    /// infra to support other rooms in the colony
    Owned,
    /// a remote mining room
    // TODO: be more granular about this, different types probably
    Remote { reserved: bool },
}
