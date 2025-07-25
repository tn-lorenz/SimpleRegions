pub mod tag;

use std::sync::Arc;
use dashmap::DashSet;
use once_cell::sync::Lazy;
use pumpkin::world::World;
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;
use crate::region::tag::Tag;

/*pub static REGIONS: Lazy<DashSet<Region>> = Lazy::new(DashSet::new);

#[derive(Clone)]
pub struct Region {
    name: String,
    world: World,
    pos1: BlockPos,
    pos2: BlockPos,
    tags: Vec<Tag>,
    children: Vec<Region>,
}

impl Region {
    pub(crate) fn new(name: String, world: World, pos1: BlockPos, pos2: BlockPos, tags: Vec<Tag>, children: Vec<Region>) -> Self {
        Self {
            name,
            world,
            pos1,
            pos2,
            tags,
            children,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn pos1(&self) -> BlockPos {
        self.pos1
    }

    pub fn pos2(&self) -> BlockPos {
        self.pos2
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn children(&self) -> &[Region] {
        &self.children
    }

    pub fn is_within(&self, location: BlockPos) -> bool {

    }
}

pub fn find_region(location: &BlockPos) -> Option<Arc<Region>> {
    for region in REGIONS.iter() {
        if region.is_within(location) {
            return Some(region.clone());
        }
    }
    None
}*/