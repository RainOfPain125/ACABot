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

mod init;

use anyhow::Result;
use rusqlite::{self, Connection};

/// Get database connection.
pub fn get_connection() -> Result<Conn> {
    let conn = Connection::open("bot.db")?;
    conn.execute(init::INIT_SQL, [])?;
    Ok(Conn { connection: conn })
}

/// Struct containing a rusqlite connection and abstract methods to interact with it.
pub struct Conn {
    pub connection: rusqlite::Connection, // TODOOO: Wrap this in arc or something so that it can be locked when in use
}

impl Conn {
    /// Update the guilds table with the current guilds the bot is in
    pub async fn update_guilds(&self, ctx: serenity::client::Context) -> Result<()> {
        for id in ctx.cache.guilds().await {
            self.connection.execute("DELETE FROM guilds WHERE *;
            INSERT INTO guilds VALUES (?)", [id.0]);
        }
        Ok(())
    }
}