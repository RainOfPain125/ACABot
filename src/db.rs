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

use std::sync::Mutex;

mod init;

use anyhow::Result;
use rusqlite::{self, Connection};
use serenity::model::id::GuildId;

/// Get database connection.
pub fn get_connection() -> Result<Mutex<Conn>> {
    let conn = Connection::open("bot.db")?;
    conn.execute(init::INIT_SQL, [])?;
    Ok(Mutex::new(Conn { connection: conn })) // TODOOO: Wrap this in arc & mutex or something so that it can be locked when in use
}

/// Struct containing a rusqlite connection and abstract methods to interact with it.
pub struct Conn {
    pub connection: rusqlite::Connection,
}


impl Conn {
    /// Update the guilds table with a vector of guild ids
    pub fn update_guilds(&self, ids: Vec<GuildId>) -> Result<()> {
        for id in ids {
            self.connection.execute(
                "DELETE FROM guilds; 
            INSERT INTO guilds VALUES (?)", // TODOO: Once guilds start having actual values this won't work.
                                            // Change so that it only romoves or adds what needs removing or adding
                rusqlite::params![id.0],
            )?;
        }
        Ok(())
    }
}
