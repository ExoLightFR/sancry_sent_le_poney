use std::{time::{Duration, SystemTime, SystemTimeError}, thread::JoinHandle, sync::Arc, error::Error};

use serenity::{prelude::Context, model::prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType, UserId, Member}, json::Value, builder::CreateApplicationCommand, http::Http};
use serde::{Serialize, Deserialize};
use tokio::task::JoinError;
use tracing::error;

use crate::get_bot_data;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MuteData {
	until: Option<std::time::SystemTime>,
	reason: Option<String>,
	from: Option<UserId>,
}

pub fn register_cmd(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd
		.name("mute")
		.description("L'admin abuse enfin rendu au peuple. Mais que sur Sancry.")
		.create_option(|opt| {
			opt.name("raison")
				.description("La raison du mute")
				.kind(CommandOptionType::String)
				.required(false)
				.max_length(127)
		})
}

async fn unmute_member_when(http: Arc<Http>, mut member: Member, when: SystemTime) -> Result<(), Box<dyn Error + Send + Sync>> {
	let time_left = when.duration_since(SystemTime::now())?;
	tokio::time::sleep(time_left).await;
	member.remove_role(http.clone(), 624698155732041755).await?;
	return Ok(());
}

pub async fn exec_mute_sancry(
	ctx: &Context,
	command: &ApplicationCommandInteraction
) -> Result<String, String>
{
	// I hate this. Isn't there a better way?
	let reason = match command.data.options.get(0) {
		Some(arg) => match &arg.value {
			Some(Value::String(reason)) => Some(reason.clone()),
			_ => None,
		},
		None => None,
	};

	let bot_data = get_bot_data(ctx).await;
	let mute_until;
	{
		let mut aboos = bot_data.aboos.write().await;
		*aboos = MuteData{
			until: Some(SystemTime::now() + Duration::from_secs(60 * 5)),
			reason,
			from: Some(command.user.id),
		};
		mute_until = aboos.until.unwrap();
	}

	let mut sancry = bot_data.get_sancry()
		.await
		.map_err(|e| e.to_string())?;
	// Mute sancry before task launch, so we can reply to user whether mute was successful or not
	sancry.add_role(bot_data.http.clone(), 624698155732041755)
		.await
		.map_err(|e| e.to_string())?;
	let res = tokio::spawn(
		unmute_member_when(ctx.http.clone(), sancry, mute_until)
	).await.map_err(|e| e.to_string())?;
	if let Err(e) = res {
		error!("Impossible de demute Sancry: {e}");
	}

	return Ok("Bon ça suffit là, chut.".into());
}
