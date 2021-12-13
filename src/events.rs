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

use super::db;

use serenity::{async_trait, model::gateway::Ready, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let conn = db::get_connection().unwrap().update_guilds(ctx);
        conn.await.unwrap();
    }
    async fn guild_unavailable(&self, ctx: Context, _: serenity::model::id::GuildId) {
        let conn = db::get_connection().unwrap().update_guilds(ctx);
        conn.await.unwrap();
    }
    async fn guild_create(&self, ctx: Context, _: serenity::model::guild::Guild, _is_new: bool) {
        let conn = db::get_connection().unwrap().update_guilds(ctx);
        conn.await.unwrap();
    }
    // TODO: Add 'add member' and 'remove member' events here to add and remove members from a member table.
    // Keep in mind a member still maybe in another server within the network so do not remove them in that case.
}