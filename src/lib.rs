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

pub mod general;

use anyhow::Result;
use serenity::model::id;

/// A struct containing user configuration derived from the .env file.
pub struct Config {
    pub log_channel: id::ChannelId,
    pub vote_channel: id::ChannelId,
}

/// Fetch an instance of Config.
/// ```no_run (cannot-doctest-environment-variable-dependency)
/// use aca_bot::get_config;
///
/// use anyhow::Result;
/// use std::env;
///
/// fn main() -> Result<()> {
///     env::set_var("LOG_CHANNEL", "00000000");
///     env::set_var("LOG_VOTE", "12345678");
///
///     let config = get_config()?;
///
///     assert_eq!(config.log_channel, 00000000);
///     assert_eq!(config.log_channel, 12345678);
///
///     Ok(())
/// }
/// ```
pub fn get_config() -> Result<Config> {
    Ok(Config {
        log_channel: id::ChannelId {
            0: env::var("LOG_CHANNEL")?.parse::<u64>()?,
        },
        vote_channel: id::ChannelId {
            0: env::var("VOTE_CHANNEL")?.parse::<u64>()?,
        },
    })
}
