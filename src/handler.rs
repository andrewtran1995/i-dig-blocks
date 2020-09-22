use crate::cubed_host::CubedHostClient;
use serenity::utils::MessageBuilder;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::EventHandler,
};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {:?}", why)
                }
            }
            _ => println!("Unknown message: {}", msg.content),
        }
    }
}

impl serenity::prelude::TypeMapKey for CubedHostClient {
    type Value = CubedHostClient;
}

#[group]
#[commands(restart_server)]
struct General;

#[command]
#[aliases("restart", "restart server", "turn me off then on")]
async fn restart_server(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Received command 'restart_server'");
    let data = ctx.data.read().await;
    let ch_client = data.get::<CubedHostClient>().unwrap();

    ch_client
        .restart_server()
        .await
        .expect("Err when restarting the Cubed Host server!");

    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            MessageBuilder::new()
                .push("User ")
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
