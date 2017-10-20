//!
//! Generate a star system
//!
pub struct System {
    name: String,
    stars: Vec<Star>,
    objects: Vec<Object>,
}

pub struct Star {
    name: String,
    class: String,
    desc: String,
}

pub struct Object {
    name: String,
    ty: Type,
    in_orbit: Vec<Object>,
}

pub enum Type {
    Planet, Belt, Gas, Cloud
}