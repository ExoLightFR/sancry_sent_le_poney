use serenity::{prelude::Context, model::prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType}, json::Value, builder::CreateApplicationCommand};

use crate::get_bot_data;

pub fn register_cmd(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd.name("rename").description("Renommez Sancry. Même sans être modo.")
		.create_option(|opt| {
			opt.name("nom")
				.description("Le nouveau nom de Sancry")
				.kind(CommandOptionType::String)
				.required(true)
				.max_length(32)
		})
}

pub async fn watashi_no_namae_ha_sankuri_desu(
	ctx: &Context,
	command: &ApplicationCommandInteraction
) -> Result<String, String>
{
	let new_name = match command.data.options.get(0) {
		Some(arg) => match &arg.value {
			Some(Value::String(choice)) => choice,
			_ => return Err("Error: missing or invalid argument".to_string()),
		},
		None => return Err("Error: invalid argument".to_string()),
	};

	let bot_data = get_bot_data(ctx).await;
	let sancry = bot_data.get_sancry().await.map_err(|x| x.to_string()).unwrap();

	match sancry.edit(ctx.http.clone(), |sancry| sancry.nickname(&new_name)).await {
		Ok(_) => Ok(format!("Sancry s'appelle maintenant {new_name}")),
		Err(e) => Err(format!("Erreur: {e}")),
	}
}
