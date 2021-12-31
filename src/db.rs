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

//! Abstract over the database connection.

use std::env;
use std::fs;
use std::sync::Mutex;

mod init;

use anyhow::{Context, Result};
use rusqlite::{self, Connection};
use serenity::model::id;

/// Struct containing a rusqlite connection and abstract methods to interact with it.
pub struct Conn {
    pub connection: Mutex<rusqlite::Connection>, // TODOOO: Wrap this in arc something so that it can be shared between threads.
}

impl Conn {
    /// Get database connection.
    pub fn new() -> Result<Conn> {
        let conn;
        if env::var("DEV")? == "true" {
            fs::remove_file(
                env::var("DATABASE_PATH")
                    .context("Couldn't open database connection. Fill out .env")?,
            )
            .ok();
            conn = Connection::open(
                env::var("DATABASE_PATH")
                    .context("Couldn't open database connection. Fill out .env")?,
            )?;
            conn.execute(init::INIT_SQL, [])?;
        } else {
            conn = Connection::open(
                env::var("DATABASE_PATH")
                    .context("Couldn't open database connection. Fill out .env")?,
            )?;
        }
        Ok(Conn {
            connection: Mutex::new(conn),
        })
    }

    /// Update the guilds table with a vector of guild ids
    pub fn update_guilds(&self, ids: Vec<id::GuildId>) -> Result<()> {
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
