use crate::cubed_host::{CubedHostClient, GetPlayersResponse};
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
    utils::MessageBuilder,
};

#[group]
#[commands(ping)]
pub struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why)
    }

    Ok(())
}

#[group]
#[commands(restart_server, get_players)]
pub struct CubedHost;

#[command]
#[aliases("restart", "turnMeOffThenOn")]
async fn restart_server(ctx: &Context, msg: &Message) -> CommandResult {
    get_cubed_host_client(ctx).await.restart_server().await?;

    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            MessageBuilder::new()
                .push_bold_safe(&msg.author.name)
                .push(" successfully restarted the server!")
                .build(),
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[aliases("players", "whoThere")]
async fn get_players(ctx: &Context, msg: &Message) -> CommandResult {
    let players: Vec<String> = get_cubed_host_client(ctx)
        .await
        .get_players()
        .await?
        .json::<GetPlayersResponse>()
        .await?
        .iter()
        .filter(|p| p.status != 0)
        .map(|p| format!("{}", p.name))
        .collect();

    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            MessageBuilder::new()
                .push(if players.is_empty() {
                    "No peeps are craftin' right now :sob:".to_string()
                } else {
                    format!(
                        "Looks like these peeps are craftin' :pick:: {}",
                        players.join(", ")
                    )
                })
                .build(),
        )
        .await
    {
        println!("Error sending message: {:?}", why)
    }

    Ok(())
}

async fn get_cubed_host_client(ctx: &Context) -> CubedHostClient {
    let data = ctx.data.read().await;
    data.get::<CubedHostClient>().unwrap().clone()
}