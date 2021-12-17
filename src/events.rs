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

use super::db;

use anyhow::Result;
use serenity::{async_trait, model::gateway::Ready, prelude::*};
pub struct Handler {
    connection: db::Conn,
}

impl Handler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connection: db::Conn::new()?,
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        handle_result(update_guilds(ctx, &self.connection).await);
    }
    async fn guild_unavailable(&self, ctx: Context, _: serenity::model::id::GuildId) {
        handle_result(update_guilds(ctx, &self.connection).await);
    }
    async fn guild_create(&self, ctx: Context, _: serenity::model::guild::Guild, _: bool) {
        handle_result(update_guilds(ctx, &self.connection).await);
    }
    // TODO: Add 'add member' and 'remove member' events here to add and remove members from a member table. Keep in mind a member still maybe in another server within the network so do not remove them in that case.
}

fn handle_result(result: Result<()>) {
    match result {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

async fn update_guilds(ctx: Context, conn: &db::Conn) -> Result<()> {
    conn.update_guilds(ctx.cache.guilds().await)?;
    Ok(())
}
