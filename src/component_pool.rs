use crate::entity_id::EntityId;
use crate::index::Index;
use crate::map::Map;
use crate::set::Set;

pub struct ComponentPool {
    pub entity_indices: Map<EntityId, Index>,
    pub entity_ids: Set<EntityId>,
}

impl ComponentPool {
    pub fn add_entities(&mut self, entity_ids: &Vec<EntityId>) {
        let new_indices = (self.entity_ids.len() as Index)
            ..(self.entity_ids.len() as Index) + (entity_ids.len() as Index);
        self.entity_ids.extend(entity_ids);
        entity_ids
            .iter()
            .zip(new_indices)
            .for_each(|(entity_id, index)| {
                self.entity_indices.insert(*entity_id, index);
            })
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entity_indices: Map::with_capacity(capacity),
            entity_ids: Set::with_capacity(capacity),
        }
    }
}
