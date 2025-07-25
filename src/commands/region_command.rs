use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

use async_trait::async_trait;

use crate::util::task_util::start_loop;

use pumpkin::command::dispatcher::CommandError::CommandFailed;
use pumpkin::{
    command::{
        CommandExecutor, CommandSender,
        args::{Arg, ConsumedArgs},
        dispatcher::{CommandError, CommandError::InvalidConsumption},
        tree::{
            CommandTree,
            builder::{argument, require},
        },
    },
    entity::player::Player,
    server::Server,
};
use pumpkin::command::args::simple::SimpleArgConsumer;
use pumpkin_util::text::TextComponent;
use crate::commands::MSG_NOT_PLAYER;
use crate::run_task_timer;

const NAMES: [&str; 2] = ["region", "rg"];
const DESCRIPTION: &str =
    "The main region command.";
const ARG_NAME: &str = "arg";

struct RegionExcecutor;

#[async_trait]
impl CommandExecutor for RegionExcecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        server: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Simple(input)) = args.get(ARG_NAME) else {
            return Err(InvalidConsumption(Some(ARG_NAME.into())));
        };

        let CommandSender::Player(player) = sender else {
            sender
                .send_message(
                    TextComponent::text(MSG_NOT_PLAYER)
                        .color_named(pumpkin_util::text::color::NamedColor::Red),
                )
                .await;

            return Err(CommandFailed(Box::new(TextComponent::text(MSG_NOT_PLAYER))));
        };

        //let input_string = input.to_string();

        //let uuid = player.gameprofile.id;

        //handle_input(player, Option::from(input_string)).await;

        Ok(())
    }
}

/*pub(crate) async fn handle_input(
    player: &Arc<Player>,
    input: Option<String>,
) {
    let Some(s) = input else {
        log::error!("Damager input is None. How did you even do this?");
        return;
    };
}*/

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_NAME, SimpleArgConsumer).execute(RegionExcecutor))
}
