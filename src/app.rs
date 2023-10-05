use std::sync::Arc;

use crate::array_view_indices::{ArrayViewIndices, MultipleArrayViewIndices};
use crate::component_id::ComponentId;
use crate::component_pool::ComponentPool;
use crate::entity_id::EntityId;
use crate::error_handlers::cannot_read;
use crate::index::Index;
use crate::map::Map;
use crate::query::Query;
use crate::query_id::QueryId;
use crate::uint32::{UInt32, UInt32Rhs};
use numpy::PyArray1;
use pyo3::prelude::*;

#[pyclass]
pub struct RustApp {
    despawned_entity_ids: Vec<EntityId>,
    next_entity_id: Index,
    // TODO: Queries should be cached by the COmponents they have
    // so that if two of the same query appear in different systems
    // it only needs to be performed once
    queries: Vec<Query>,
    pools: Map<ComponentId, ComponentPool>,
    entity_id_component: ComponentId,
    entity_ids: UInt32,
}

#[pymethods]
impl RustApp {
    fn spawn(
        &mut self,
        components: Vec<ComponentId>,
        num: Index,
    ) -> PyResult<Vec<ArrayViewIndices>> {
        let num_drain = (self.despawned_entity_ids.len() as u32).min(num);
        let num_left = num - num_drain;
        let entity_ids: Vec<_> = self
            .despawned_entity_ids
            .drain(..num_drain as usize)
            .chain(self.next_entity_id..self.next_entity_id + num_left)
            .collect();
        self.next_entity_id += num;

        let mut result = Vec::with_capacity(components.len());
        let mut spawned_entity_ids = None;
        for (i, component) in components.into_iter().enumerate() {
            let indices = self.pools.get_mut(&component).unwrap().spawn(&entity_ids)?;
            if component == self.entity_id_component {
                spawned_entity_ids = Some(i);
            }
            result.push(indices);
        }

        if let Some(index) = spawned_entity_ids {
            self.entity_ids
                .p_new_view_with_indices(unsafe { result.get_unchecked(index) })
                .fill(UInt32Rhs::VecU32(entity_ids))?;
        } else {
            let indices = self
                .pools
                .get_mut(&self.entity_id_component)
                .unwrap()
                .spawn(&entity_ids)?;
            self.entity_ids
                .p_new_view_with_indices(&indices)
                .fill(UInt32Rhs::VecU32(entity_ids))?;
        }

        Ok(result)
    }

    fn despawn(&mut self, entity_ids: &UInt32) -> PyResult<()> {
        // let array = entity_ids.array.read().map_err(cannot_read)?;
        // let indices = entity_ids.indices.0.read().map_err(cannot_read)?;
        // for &index in indices.iter() {
        //     let entity_id = unsafe { array.get_unchecked(index as usize) };
        //     self.despawned_entity_ids.push(*entity_id);
        //     for pool in self.pools.values_mut() {
        //         pool.despawn(*entity_id);
        //     }
        // }
        Ok(())
    }

    #[staticmethod]
    fn new(
        entity_id_component: ComponentId,
        num_entities: usize,
        num_pools: usize,
        num_queries: usize,
    ) -> PyResult<(Self, UInt32)> {
        let mut pools = Map::with_capacity(num_pools);
        let indices = ArrayViewIndices::with_capacity(num_entities);
        pools.insert(entity_id_component, ComponentPool::from_indices(&indices)?);
        let entity_ids = UInt32::p_from_indices(&indices, 0)?;
        Ok((
            Self {
                next_entity_id: 0,
                despawned_entity_ids: Vec::new(),
                queries: Vec::with_capacity(num_queries),
                pools,
                entity_id_component,
                entity_ids: entity_ids.p_new_view_with_indices(&indices),
            },
            entity_ids,
        ))
    }

    fn add_pool(&mut self, component_id: ComponentId, capacity: usize) -> ArrayViewIndices {
        ArrayViewIndices(Arc::clone(
            &self
                .pools
                .entry(component_id)
                .or_insert(ComponentPool::with_capacity(capacity))
                .array_view_indices
                .0,
        ))
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
