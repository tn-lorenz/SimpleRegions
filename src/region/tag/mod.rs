use std::time::Duration;

pub struct Tag {
    name: String,
    tag_type: RegionTagType,
    activation_types: Vec<ActivationType>,
}

pub enum RegionTagType {
    Bool(bool),
    Attribute,
}

pub enum ActivationType {
    OnEnter,
    OnLeave,
    WhileInside,
}

pub trait ApplyContinuously {
    async fn continuously(&self, delay: Duration);
}

pub trait Greeting {
    async fn greet(&self);
}

pub trait Farewell {
    async fn farewell(&self);
}