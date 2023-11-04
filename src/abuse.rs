use serenity::{prelude::Context, model::prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType}, json::Value, builder::CreateApplicationCommand};

use crate::get_bot_data;

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

pub async fn mute_sancry(
	ctx: &Context,
	command: &ApplicationCommandInteraction
) -> Result<String, String>
{
	return Ok("foo".into());
}
