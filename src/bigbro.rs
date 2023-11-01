use std::str::FromStr;

use serenity::{prelude::*, model::prelude::{Message, ReactionType}};
use tracing::error;

use crate::Bot;



pub async fn big_brother_is_watching(bot: &Bot, ctx: &Context, msg: &Message) {
	if msg.author.id != bot.sancry_id {
		return;
	}
	if msg.content.to_lowercase().contains("exo") {
		let wags = match ReactionType::from_str("<:Wags:773559519061999646>") {
			Ok(res) => res,
			Err(err) => { error!("Failed to convert emoji: {err}"); return; },
		};
		
		if let Err(e) = msg.react(ctx.http.clone(), wags).await {
			error!("Failed to react to message: {e}");
		}
	}
}
