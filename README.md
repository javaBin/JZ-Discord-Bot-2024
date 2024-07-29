# JZ-discord-bot
RustBot is a simple Discord bot written in Rust. It provides various utility commands for the javazone discord server.

## structure
- **Main**: Contains the main functionality of the bot.
- **Discord**: Handles Discord-related operations and interactions.
- **Utils**: Provides utility functions and models.

## Features
The bot is planned to have these commands:
- `/signup` -- For volunteers to sign up and receive their role. This will be validated on an internally stored list of pre registered volunteers.
- `/next` -- Fetches the next talks on the program, and displays them in a compact, tidy way.
- `/announce`  -- allows the bot to send messages with @everyone in the announcement channel.


## Running the bot
To run the bot, you need to have Rust installed on your system. Follow these steps to get started:
1. The bot relies on secrets from .env, mainly this includes these fields:
 * DISCORD_TOKEN
 * DISCORD_CLIENT_ID
 * SESSION_URL_PREFIX
 * PROGRAM_URL
 * ANNOUNCEMENT_WEBHOOK
 * VOLUNTEER_SOURCEFILE
2. Build the project: `cargo build`
3. Run the bot: `cargo run`


## Usage

Once the bot is running, you can invite it to your Discord server and start using its commands. 
    - Slash commands are deployed to discord when bot is started.


