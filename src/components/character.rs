use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Character {
    /* Doesn't contain data yet, this is just a marker */
}

impl Character {
    pub fn new() -> Character {
        Character {}
    }
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}
