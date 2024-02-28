use std::{collections::HashMap, sync::LazyLock};

use screeps::{ResourceType, StructureObject, StructureProperties, StructureType, RESOURCES_ALL};

/// the set of resources that each structure can store
pub static STRUCTURE_STORE_RESOURCES: LazyLock<HashMap<StructureType, &[ResourceType]>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert(StructureType::Container, RESOURCES_ALL);
        map.insert(StructureType::Extension, &[ResourceType::Energy]);
        map.insert(StructureType::Factory, RESOURCES_ALL);
        map.insert(StructureType::Lab, RESOURCES_ALL);
        map.insert(StructureType::Link, &[ResourceType::Energy]);
        map.insert(
            StructureType::Nuker,
            &[ResourceType::Energy, ResourceType::Ghodium],
        );
        map.insert(
            StructureType::PowerSpawn,
            &[ResourceType::Energy, ResourceType::Power],
        );
        map.insert(StructureType::Spawn, &[ResourceType::Energy]);
        map.insert(StructureType::Storage, RESOURCES_ALL);
        map.insert(StructureType::Terminal, RESOURCES_ALL);
        map.insert(StructureType::Tower, &[ResourceType::Energy]);

        map
    });

pub trait StructureExt {
    /// returns the possible resource types that this structure can hold. if the structure cannot
    /// hold any resources (it doesn't have a Store), an empty slice is returned.
    fn resource_types(&self) -> &[ResourceType];
}

impl StructureExt for StructureObject {
    fn resource_types(&self) -> &[ResourceType] {
        STRUCTURE_STORE_RESOURCES
            .get(&self.structure_type())
            .copied()
            .unwrap_or(&[])
    }
}
