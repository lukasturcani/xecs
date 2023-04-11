use crate::array::Array;
use crate::entity_id::EntityId;
use crate::entity_index::EntityIndex;
use crate::map::Map;
use crate::set::Set;

pub struct ComponentPool<T> {
    entity_indices: Map<EntityId, EntityIndex>,
    entity_ids: Set<EntityId>,
    components: Array<T>,
}
