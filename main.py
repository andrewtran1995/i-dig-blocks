import configparser
import discord


def main():
    config = configparser.ConfigParser()
    config.read("project.properties")
    discord_bot_token = config.get("Discord", "BotToken")

    client = discord.Client()

    @client.event
    async def on_ready():
        print(f'{client.user} has connected to Discord!')

    client.run(discord_bot_token)


if __name__ == '__main__':
    main()
