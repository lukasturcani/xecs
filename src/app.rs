use crate::array_view_indices::MultipleArrayViewIndices;
use crate::component_id::ComponentId;
use crate::component_pool::ComponentPool;
use crate::entity_id::EntityId;
use crate::index::Index;
use crate::map::Map;
use crate::query::Query;
use crate::query_id::QueryId;
use pyo3::prelude::*;

#[pyclass]
pub struct RustApp {
    num_spawned_entities: Index,
    // TODO: Queries should be cached by the COmponents they have
    // so that if two of the same query appear in different systems
    // it only needs to be performed once
    queries: Vec<Query>,
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
    fn __new__(num_pools: usize, num_queries: usize) -> Self {
        Self {
            num_spawned_entities: 0,
            pools: Map::with_capacity(num_pools),
            queries: Vec::with_capacity(num_queries),
        }
    }

    fn add_pool(&mut self, component_id: ComponentId, capacity: usize) {
        self.pools
            .insert(component_id, ComponentPool::with_capacity(capacity));
    }

    fn add_query(
        &mut self,
        first_component: ComponentId,
        other_components: Vec<ComponentId>,
    ) -> QueryId {
        self.queries
            .push(Query::new(first_component, other_components));
        self.queries.len() - 1
    }

    fn run_query(&self, query_id: QueryId) -> MultipleArrayViewIndices {
        unsafe { self.queries.get_unchecked(query_id) }.result(&self.pools)
    }
}
