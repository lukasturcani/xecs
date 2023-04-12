use crate::component_id::ComponentId;
use crate::component_pool::ComponentPool;
use crate::entity_id::EntityId;
use crate::index::Index;
use crate::map::Map;
use pyo3::prelude::*;

#[pyclass]
pub struct RustApp {
    num_spawned_entities: Index,
    pools: Map<ComponentId, ComponentPool>,
}

#[pymethods]
impl RustApp {
    fn spawn(&mut self, components: Vec<ComponentId>, num: Index) {
        let entity_ids = (self.num_spawned_entities..self.num_spawned_entities + num)
            .map(EntityId)
            .collect();
        self.num_spawned_entities += num;

        components.iter().for_each(|component_id| {
            if let Some(pool) = self.pools.get_mut(component_id) {
                pool.add_entities(&entity_ids);
            }
        });
    }

    #[new]
    fn __new__() -> Self {
        Self {
            num_spawned_entities: 0,
            pools: Map::new(),
        }
    }

    fn add_component_pool(&mut self, component_id: ComponentId, capacity: usize) {
        self.pools
            .insert(component_id, ComponentPool::with_capacity(capacity));
    }
}
