use crate::component_id::ComponentId;
use crate::component_pool::ComponentPool;
use crate::index::Index;
use crate::map::Map;
use pyo3::prelude::*;

#[pyclass]
pub struct RustQuery {
    first_component: ComponentId,
    other_components: Vec<ComponentId>,
}

impl RustQuery {
    pub fn result(&self, pools: &Map<ComponentId, ComponentPool>) -> Vec<Vec<Index>> {
        let first_component = pools.get(&self.first_component).unwrap();
        let other_components: Vec<_> = self
            .other_components
            .iter()
            .map(|component_id| pools.get(component_id).unwrap())
            .collect();

        // TODO: You can probabably cache the intersection. If a component
        // is added or removed the cache can be cleared.
        // There is probably also an intelligent way of updating the intersection
        // if a component is added or removed -- rather than recalculating it
        // from scratch. This is because you know which queries are affected when
        // a component is added or removed from an entity the query intersection
        // only needs to be updated with those entities.
        let intersection = other_components
            .iter()
            .map(|pool| &pool.entity_ids)
            .fold(first_component.entity_ids.clone(), |acc, entity_ids| {
                acc.intersection(entity_ids).map(|x| *x).collect()
            });
        let mut result = Vec::with_capacity(other_components.len() + 1);
        result.push(
            intersection
                .iter()
                .map(|entity_id| *first_component.entity_indices.get(entity_id).unwrap())
                .collect(),
        );
        result.extend(other_components.iter().map(|pool| {
            intersection
                .iter()
                .map(|entity_id| *pool.entity_indices.get(entity_id).unwrap())
                .collect()
        }));
        result
    }
}

#[pymethods]
impl RustQuery {
    #[new]
    fn __new__(first_component: ComponentId, other_components: Vec<ComponentId>) -> Self {
        Self {
            first_component,
            other_components,
        }
    }
}