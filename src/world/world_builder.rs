// the parent class to create other parts of the map/world 
pub trait world_builder<T> {
    fn spawn_event(&self) -> T;
    fn spawn_item(&self)  -> T;
    fn spawn_npc(&self)   -> T;
}

