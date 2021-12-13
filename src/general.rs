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

//! General purpose isolated commands that do not affect state or the such

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

/// Repeat everything the caller said after the sixth index position or rather everything after "!echo ".
#[command]
pub async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, &msg.content_safe(ctx).await[6..])
        .await?;
    Ok(())
}
