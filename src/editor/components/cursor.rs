use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Clone, Default, Debug)]
pub struct Cursor;

impl Component for Cursor {
    type Storage = NullStorage<Cursor>;
}
