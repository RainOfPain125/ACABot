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

pub struct Config {
    LogChannel: id::ChannelId,
    VoteChannel: id::ChannelId,
}

pub fn get_config() -> Result<Config> {
    Ok(Config {
        LogChannel: id::ChannelId {
            0: env::var("LOG_CHANNEL")?.parse::<u64>()?,
        },
        VoteChannel: id::ChannelId {
            0: env::var("VOTE_CHANNEL")?.parse::<u64>()?,
        },
    })
}
