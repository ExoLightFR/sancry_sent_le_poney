use std::{str::FromStr, sync::Arc, error::Error};

use serenity::{prelude::*, model::prelude::{Message, ReactionType, Presence, ActivityType, GuildId}};
use tracing::{error, info};

use crate::BotData;

// use crate::Bot;



pub async fn big_brother_is_watching(bot: &Arc<BotData>, ctx: &Context, msg: &Message) {
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

// async fn check_sancry_LoL(&self, ctx: &Context, data: &Presence) -> Result<(), Box<dyn Error>> {
// 	if data.user.id != self.sancry_id {
// 		return Ok(());
// 	}
// 	let mut activities = data.activities.iter()
// 		.filter(|x| x.kind == ActivityType::Playing || x.kind == ActivityType::Competing);
// 	if activities.any(|x| x.name == "League of Legends") {
// 		info!("ATTENTION!!! SANCRY JOUE A LOL");
// 		let sancry = GuildId::member(self.guild_id, ctx.http.clone(), self.sancry_id)
// 			.await?;
// 		for _ in 1..10 {
// 			sancry.user.direct_message(ctx.http.clone(), |m| {
// 				m.content("WTF SANCRY ARRÊTE DE JOUER À CE JEU DE CON TOUT DE SUITE")
// 			}).await?;
// 		}
// 	}
// 	return Ok(());
// }

pub async fn check_sancry_jeu_de_con(
	bot_data: &Arc<BotData>,
	ctx: &Context,
	presence: &Presence
) -> Result<(), Box<dyn Error>>
{
	if presence.user.id != bot_data.sancry_id {
		return Ok(());
	}
	let mut activities = presence.activities
		.iter()
		.filter(|x| x.kind == ActivityType::Playing || x.kind == ActivityType::Competing);

	if activities.any(|x| x.name == "League of Legends") {
		info!("ATTENTION!!! SANCRY JOUE A LOL");
		let sancry = GuildId::member(bot_data.guild_id, ctx.http.clone(), bot_data.sancry_id)
			.await?;
		for _ in 1..10 {
			sancry.user.direct_message(ctx.http.clone(), |m| {
				m.content("WTF SANCRY ARRÊTE DE JOUER À CE JEU DE CON TOUT DE SUITE")
			}).await?;
		}
	}
	return Ok(());
}
