// the abstract parent class to create other parts of the map/world 
pub trait AbstractWorldBuilder<T> {
    fn spawn_event(&self) -> T;
    fn spawn_item(&self)  -> T;
    fn spawn_npc(&self)   -> T;
    fn roll_dice(&self)   -> T;
}
