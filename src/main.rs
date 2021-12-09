// This file is part of aca_bot.

// aca_bot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// aca_bot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with aca_bot.  If not, see <https://www.gnu.org/licenses/>.

//! A Discord bot focused on addressing the inherent problems with Discord, to allow a more socialist/anarchist organization of servers (or "guilds").

use std::env;

mod lib;

use anyhow::Result;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    Client,
};

// Groups
#[group]
#[commands("echo")]
struct General;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    // Serenity boilerplate
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(env::var("DISCORD_TOKEN")?)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
