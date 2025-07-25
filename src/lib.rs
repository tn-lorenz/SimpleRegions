use crate::listeners::player_move::PlayerMoveHandler;
use once_cell::sync::Lazy;
use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::{
    PermissionLvl,
    permission::{Permission, PermissionDefault},
};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub mod commands;
mod events;
pub mod listeners;
mod region;
mod util;

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
const REGION_PERMISSION_NODE: String = format!("{PLUGIN_NAME}:command.region");

pub static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create global Tokio Runtime"));

async fn register_commands(context: &Context) -> Result<(), String> {
    let region_cmd_permission = Permission::new(
        &REGION_PERMISSION_NODE,
        "Grants access to the /region command.",
        PermissionDefault::Op(PermissionLvl::Four),
    );

    context.register_permission(region_cmd_permission).await?;

    context
        .register_command(
            commands::region_command::init_command_tree(),
            &REGION_PERMISSION_NODE,
        )
        .await;

    Ok(())
}

async fn register_events(context: &Context) {
    context
        .register_event(Arc::new(PlayerMoveHandler), EventPriority::Lowest, true)
        .await;
}

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    register_commands(server).await?;
    register_events(server).await;

    log::info!("SimpleRegions has been loaded.");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
