use std::sync::Arc;

use pyo3::PyResult;

use crate::array_view_indices::ArrayViewIndices;
use crate::entity_id::EntityId;
use crate::error_handlers::cannot_read;
use crate::index::Index;
use crate::map::Map;
use crate::set::Set;

pub struct ComponentPool {
    pub entity_indices: Map<EntityId, Index>,
    pub entity_ids: Set<EntityId>,
    pub array_view_indices: ArrayViewIndices,
}

impl ComponentPool {
    pub fn spawn(&mut self, entity_ids: &[EntityId]) -> PyResult<ArrayViewIndices> {
        let new_indices = (self.entity_ids.len() as Index)
            ..(self.entity_ids.len() as Index) + (entity_ids.len() as Index);
        self.entity_ids.extend(entity_ids);
        entity_ids
            .iter()
            .zip(new_indices)
            .for_each(|(entity_id, index)| {
                self.entity_indices.insert(*entity_id, index);
            });
        self.array_view_indices.spawn(entity_ids.len() as Index)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entity_indices: Map::with_capacity(capacity),
            entity_ids: Set::with_capacity(capacity),
            array_view_indices: ArrayViewIndices::with_capacity(capacity),
        }
    }
    pub fn from_indices(indices: &ArrayViewIndices) -> PyResult<Self> {
        let capacity = indices.0.read().map_err(cannot_read)?.capacity();
        Ok(Self {
            entity_indices: Map::with_capacity(capacity),
            entity_ids: Set::with_capacity(capacity),
            array_view_indices: ArrayViewIndices(Arc::clone(&indices.0)),
        })
    }
}
