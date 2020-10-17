use std::collections::HashSet;

use chrono::Local;
use serenity::{
    client::Context,
    framework::standard::macros::{help, hook},
    framework::standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions},
    model::channel::Message,
    model::prelude::UserId,
};

use crate::cubed_host::CubedHostClient;

#[help]
#[command_not_found_text = "Could not find: '{}. :x:"]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn before(_: &Context, _: &Message, command_name: &str) -> bool {
    println!("[{}] Received command: {}", Local::now(), command_name);
    true
}

impl serenity::prelude::TypeMapKey for CubedHostClient {
    type Value = CubedHostClient;
}
