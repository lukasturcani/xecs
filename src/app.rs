use crate::component_id::ComponentId;
use crate::component_pool::ComponentPool;
use crate::index::Index;

pub struct App {
    pools: Vec<ComponentPool>,
}

impl App {
    fn add_component_pool(&mut self, component_id: ComponentId, size: Index) {
        if let Some(pool) = self.pools.get(component_id as usize) {}
    }
}
