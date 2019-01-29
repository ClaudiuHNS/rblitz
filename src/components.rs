use specs::{Component, DenseVecStorage, VecStorage};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Health(f32);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct NetId(u32);

impl Component for NetId {
    type Storage = DenseVecStorage<Self>;
}
