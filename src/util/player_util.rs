use crate::region::Region;
use async_trait::async_trait;
use pumpkin::entity::player::Player;
use std::sync::Arc;

pub struct PlayerAttributes {
    pub current: Option<Arc<Region>>,
}

impl PlayerAttributes {
    pub fn new() -> PlayerAttributes {
        Self { current: None }
    }

    pub async fn get_current(&self) -> Option<Arc<Region>> {
        if Some(self.current.clone()) {
            self.current.clone()
        } else {
            log::warn!("Not currently standing in any region.");
            None
        }
    }
}

#[async_trait]
#[allow(dead_code)]
pub(crate) trait PlayerUtil {}

#[async_trait]
#[allow(dead_code)]
impl PlayerUtil for Arc<Player> {}
