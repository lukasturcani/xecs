use crate::entity_id::EntityId;
use crate::entity_index::EntityIndex;
use crate::map::Map;
use crate::set::Set;

pub struct Query<'pool> {
    pub first_component_entity_ids: &'pool Set<EntityId>,
    pub component_entity_ids: Vec<&'pool Set<EntityId>>,
    pub first_component_entity_indices: &'pool Map<EntityId, EntityIndex>,
    pub component_entity_indices: Vec<&'pool Map<EntityId, EntityIndex>>,
}

impl<'pool> Query<'pool> {
    fn result(&self) -> Vec<Vec<EntityIndex>> {
        // TODO: You can probabably cache the intersection. If a component
        // is added or removed the cache can be cleared.
        // There is probably also an intelligent way of updating the intersection
        // if a component is added or removed -- rather than recalculating it
        // from scratch. This is because you know which queries are affected when
        // a component is added or removed from an entity the query intersection
        // only needs to be updated with those entities.
        let intersection = *self
            .component_entity_ids
            .iter()
            .fold(self.first_component_entity_ids, |acc, &x| {
                &acc.intersection(x).map(|x| *x).collect()
            });
        let result = Vec::with_capacity(self.first_component_entity_indices.len() + 1);
        result.push(
            intersection
                .iter()
                .map(|entity_id| *self.first_component_entity_indices.get(entity_id).unwrap())
                .collect(),
        );
        result.extend(self.component_entity_indices.iter().map(|entity_indices| {
            intersection
                .iter()
                .map(|entity_id| *entity_indices.get(entity_id).unwrap())
                .collect()
        }));
        result
    }
}
