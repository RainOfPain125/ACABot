// A Discord bot focused on addressing the inherent problems with Discord, to allow a more socialist/anarchist organization of servers (or "guilds").
// Copyright (C) 2021 ACA

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;

mod db;
mod events;
use aca_bot::general::*;

use anyhow::{Context, Result};
use serenity::{
    client::{bridge::gateway::GatewayIntents},
    framework::{standard::macros::group, StandardFramework},
    Client,
};
use ctrlc;

// Groups
#[group]
#[commands("echo")]
struct General;

#[tokio::main]
async fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        std::process::exit(0); // TODO: I notice that when ctrl+c is called it doesn't actually exit. This works but definitely isn't a good way of doing this. Make this exit gracefully out of tokio.
    })?;

    dotenv::dotenv()?;

    // Serenity boilerplate
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(
        env::var("DISCORD_TOKEN").context("Couldn't authenticate with Discord. Fill out .env")?,
    )
    .framework(framework)
    .intents(
        GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_BANS
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::DIRECT_MESSAGE_REACTIONS,
    )
    .event_handler(events::Handler::new()?)
    .await?;
    client.start().await?;

    Ok(())
}
