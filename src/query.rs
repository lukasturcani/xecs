use crate::component_pool::ComponentPool;
use crate::index::Index;

pub struct Query<'pool> {
    first_component: &'pool ComponentPool,
    other_components: Vec<&'pool ComponentPool>,
}

impl<'pool> Query<'pool> {
    fn result(&self) -> Vec<Vec<Index>> {
        // TODO: You can probabably cache the intersection. If a component
        // is added or removed the cache can be cleared.
        // There is probably also an intelligent way of updating the intersection
        // if a component is added or removed -- rather than recalculating it
        // from scratch. This is because you know which queries are affected when
        // a component is added or removed from an entity the query intersection
        // only needs to be updated with those entities.
        let intersection = self
            .other_components
            .iter()
            .map(|pool| pool.entity_ids)
            .fold(self.first_component.entity_ids, |acc, entity_ids| {
                acc.intersection(&entity_ids).map(|x| *x).collect()
            });
        let result = Vec::with_capacity(self.other_components.len() + 1);
        result.push(
            intersection
                .iter()
                .map(|entity_id| *self.first_component.entity_indices.get(entity_id).unwrap())
                .collect(),
        );
        result.extend(self.other_components.iter().map(|pool| {
            intersection
                .iter()
                .map(|entity_id| *pool.entity_indices.get(entity_id).unwrap())
                .collect()
        }));
        result
    }
}
