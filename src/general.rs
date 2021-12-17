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

//! General purpose isolated commands that do not affect state or the such

use std::env;

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

#[command]
#[aliases(code, source_code, git, repo)]
pub async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, env::var("REPOSITORY_LINK")?)
        .await?;
    Ok(())
}