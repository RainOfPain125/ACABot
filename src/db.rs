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

use std::env;
use std::sync::Mutex;

mod init;

use anyhow::Result;
use rusqlite::{self, Connection};
use serenity::model::id::GuildId;
/// Struct containing a rusqlite connection and abstract methods to interact with it.
pub struct Conn {
    pub connection: Mutex<rusqlite::Connection>, // TODOOO: Wrap this in arc something so that it can be shared between threads.
}

impl Conn {
    /// Get database connection.
    pub fn new() -> Result<Conn> {
        let conn = Connection::open(env::var("DATABASE_URL")?)?;
        conn.execute(init::INIT_SQL, [])?; // TODOO: This should be conditional on if the db exists or not. Maybe dev flag in .env to delete the database on startup?
        Ok(Conn {
            connection: Mutex::new(conn),
        })
    }

    /// Update the guilds table with a vector of guild ids
    pub fn update_guilds(&self, ids: Vec<GuildId>) -> Result<()> {
        for id in ids {
            self.connection.lock().unwrap().execute(
                "DELETE FROM guilds; 
            INSERT INTO guilds VALUES (?)", // TODOO: Once guilds start having actual values this won't work. Change so that it only removes or adds what needs removing or adding.
                rusqlite::params![id.0], // TODOOOO: "Wrong number of parameters passed to query. Got 1, needed 0" I'm fairly sure I did this right. Is `?` not the right sign? WTF
            )?;
        }
        Ok(())
    }
}
