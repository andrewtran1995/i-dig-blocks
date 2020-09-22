mod cubed_host;
mod handler;
mod secrets;

use crate::cubed_host::CubedHostClient;
use crate::handler::GENERAL_GROUP;
use serenity::{framework::StandardFramework, model::id::UserId, Client};

#[tokio::main]
async fn main() {
    let mut client = {
        let s = secrets::Secrets::new().expect("Error parsing 'secrets.toml'");
        println!("Parsed secrets are:");
        println!("{:}", serde_json::to_string_pretty(&s).unwrap());

        let framework = StandardFramework::new()
            .configure(|c| {
                c.on_mention(Some(UserId(s.discord.client_id)))
                    .with_whitespace(true)
            })
            .group(&GENERAL_GROUP);

        let client = Client::new(s.discord.bot_token)
            .event_handler(handler::Handler)
            .framework(framework)
            .await
            .expect("Err creating client");

        let ch_client = CubedHostClient {
            server_id: s.cubed_host.server_id,
            api_key: s.cubed_host.api_key,
            api_user: s.cubed_host.api_user,
        };
        // Run a health check.
        {
            let body = ch_client
                .get_server_config()
                .await
                .expect("Err getting server config from Cubed Host")
                .text()
                .await
                .expect("Err parsing response from Cubed Host");
            println!(
                "Server health check:\n{:}",
                json::parse(body.as_str())
                    .expect("Err parsing response from Cubed Host as JSON")
                    .pretty(4)
            );
        }

        {
            let mut data = client.data.write().await;
            data.insert::<CubedHostClient>(ch_client);
        }

        client
    };

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
