use std::error::Error;

use serenity::{client::Context, model::{application::{interaction::application_command::ApplicationCommandInteraction, command::CommandOptionType}, guild::Member, id::GuildId}, builder::CreateApplicationCommand};

use crate::{get_bot_data, orm, cmd_utils::flatten_cmd_data_option};

pub fn register_force_name(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd.name("forcename").description("haha funi")
		.create_option(|opt| {
			opt.name("user")
				.description("Target user")
				.kind(CommandOptionType::User)
				.required(true)
		})
		.create_option(|opt| {
			opt.name("name")
				.description("yeeee")
				.kind(CommandOptionType::String)
				.required(false)
				.max_length(32)
		})
}

pub fn register_unforce_name(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd.name("unforce").description("trucmuche")
		.create_option(|opt| {
			opt.name("user")
				.description("Target user")
				.kind(CommandOptionType::User)
				.required(true)
		})
}

pub async fn exec_force_name(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String, String> {
	let bot_data = get_bot_data(&ctx).await;
	let tgt_user_id = flatten_cmd_data_option(command, 0);
	let new_name = flatten_cmd_data_option(command, 1);
	if command.guild_id.is_none() {
		return Err("Not in a server".into());
	}
	if !command.member.as_ref().unwrap().permissions.unwrap().administrator() {
		return Err("You're not an admin!".into());
	}

	let tgt_user_id = tgt_user_id.ok_or("No user given!".to_string())?.as_str().unwrap();
	let new_name = match new_name {
		None => "",
		Some(x) => x.as_str().unwrap(),
	};

	sqlx::query("INSERT INTO users (user_id, guild_id, forced_name) VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT users_pk
			DO UPDATE SET forced_name = $2")
		.bind(tgt_user_id)
		.bind(command.guild_id.unwrap().to_string())
		.bind(new_name)
		.execute(&bot_data.db)
		.await
		.map_err(|e| e.to_string())?;

	let tgt_user = GuildId::member(command.guild_id.unwrap(),
		ctx.http.clone(),
		tgt_user_id.parse::<u64>().unwrap())
		.await
		.map_err(|e| e.to_string())?;
	tgt_user.edit(ctx.http.clone(), |usr| usr.nickname(new_name))
		.await
		.map_err(|e| e.to_string())?;
	return Ok("OK".into());
}

pub async fn exec_unforce_name(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String, String> {
	let bot_data = get_bot_data(&ctx).await;
	let tgt_user_id = flatten_cmd_data_option(command, 0);
	if command.guild_id.is_none() {
		return Err("Not in a server".into());
	}
	if !command.member.as_ref().unwrap().permissions.unwrap().administrator() {
		return Err("You're not an admin!".into());
	}

	let tgt_user_id = tgt_user_id.ok_or("No user given!".to_string())?.as_str().unwrap();
	let tgt_user_id: u64 = tgt_user_id.parse().unwrap();

	sqlx::query("INSERT INTO users (user_id, guild_id) VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT users_pk
			DO UPDATE SET forced_name = NULL")
		.bind(tgt_user_id.to_string())
		.execute(&bot_data.db)
		.await
		.map_err(|e| e.to_string())?;

	return Ok("Ouais, liberté ou un truc du genre là".into());
}

pub async fn toi_tu_restes_comme_ca(
	ctx: &Context,
	_old_if_available: &Option<Member>,
	new: &Member
) -> Result<(), Box<dyn Error>>
{
	let bot_data = get_bot_data(&ctx).await;
	if new.guild_id != bot_data.guild_id {
		return Ok(());
	}
	let user_db: Option<orm::User> = sqlx::query_as("SELECT * FROM users WHERE user_id = $1 AND guild_id = $2")
		.bind(new.user.id.to_string())
		.bind(new.guild_id.to_string())
		.fetch_optional(&bot_data.db)
		.await?;
	if user_db.is_none() || user_db.as_ref().unwrap().forced_name.is_none() {
		return Ok(());
	}
	let user_db = user_db.unwrap();
	let forced_name = user_db.forced_name.unwrap();
	if new.display_name().as_str() == forced_name {
		return Ok(());
	}
	let member = match ctx.cache.member(bot_data.guild_id, new.user.id) {
		Some(m) => m,
		None => GuildId::member(bot_data.guild_id, ctx.http.clone(), new.user.id).await?,
	};
	member.edit(ctx.http.clone(), |x| x.nickname(forced_name)).await?;
	return Ok(());
}
