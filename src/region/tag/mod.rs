use std::time::Duration;

#[derive(Clone)]
pub struct Tag {
    name: String,
    tag_type: RegionTagType,
    activation_types: Vec<ActivationType>,
}

#[derive(Clone)]
pub enum RegionTagType {
    Bool(bool),
    Attribute,
}

#[derive(Clone)]
pub enum ActivationType {
    OnEnter,
    OnLeave,
    WhileInside,
}

pub trait ApplyContinuously {
    async fn continuously(&self, delay: Duration);
}

pub trait Enter {
    async fn on_enter(&self);
}

pub trait Leave {
    async fn on_leave(&self);
}
