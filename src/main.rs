mod cubed_host;
mod secrets;

use crate::cubed_host::CubedHostClient;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::{Context, EventHandler},
    Client,
};

struct Handler<'a> {
    ch_client: CubedHostClient<'a>,
}

#[async_trait]
impl EventHandler for Handler<'_> {
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

#[tokio::main]
async fn main() {
    let s = secrets::Secrets::new().expect("Error parsing 'secrets.toml'");
    println!("{:?}", s);

    let mut client = Client::new(s.discord.bot_token)
        .event_handler(Handler {
            ch_client: CubedHostClient {
                server_id: &s.cubed_host.server_id,
                api_key: &s.cubed_host.api_key,
                api_user: &s.cubed_host.api_user,
            },
        })
        .await
        .expect("Err creating client");

    // Health check.
    CubedHostClient {
        server_id: &s.cubed_host.server_id,
        api_key: &s.cubed_host.api_key,
        api_user: &s.cubed_host.api_user,
    }
    .get_server_config();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
