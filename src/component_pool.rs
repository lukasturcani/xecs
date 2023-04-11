use crate::entity_id::EntityId;
use crate::index::Index;
use crate::map::Map;
use crate::set::Set;

pub struct ComponentPool {
    pub entity_indices: Map<EntityId, Index>,
    pub entity_ids: Set<EntityId>,
}
