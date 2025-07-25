use std::sync::Arc;
use async_trait::async_trait;
use pumpkin::plugin::EventHandler;
use pumpkin::plugin::player::player_move::PlayerMoveEvent;
use pumpkin::server::Server;
use pumpkin_api_macros::with_runtime;

pub struct PlayerMoveHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerMoveEvent> for PlayerMoveHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerMoveEvent) {

        if event.from == event.to { return; }

        let player = &event.player;

    }
}