use anyhow::anyhow;
use serenity::model::prelude::{Interaction, InteractionResponseType, Presence};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot
{
	guild_id: GuildId,
	sancry_id: u64,
}

#[async_trait]
impl EventHandler for Bot {
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content.to_lowercase() == "sancry" {
			info!("foo");
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
			commands.create_application_command(|command| { command.name("hello").description("Se présente") })
		}).await.unwrap();
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		// check if the interaction is a command
		if let Interaction::ApplicationCommand(command) = interaction {
 
			let response_content =
				match command.data.name.as_str() {
					"hello" => "Salut. Je suis un bot créé dans le seul et unique but de faire chier Sancry. À suivre.",
					command => unreachable!("Unknown command: {}", command),
				};
			// send `response_content` to the discord server
			command.create_interaction_response(&ctx.http, |response| {
				response
					.kind(InteractionResponseType::ChannelMessageWithSource)
					.interaction_response_data(|message| message.content(response_content))
			})
				.await.expect("Cannot respond to slash command");
		}
	}

	async fn presence_update(&self, _ctx: Context, _new_data: Presence) {
		if _new_data.user.id != self.sancry_id {
			return;
		}
		if _new_data.activities.iter().any(|x| x.name == "League of Legends") {
			info!("ATTENTION!!! SANCRY JOUE A LOL");
			// let channels = GuildId::channels(self.guild_id, _ctx.http.clone()).await.unwrap();
			// let (_, chan) = channels.iter().filter(|(_, chan)| chan.name == "🔈vocal-et-sans-micros").last().unwrap();
			// chan.say(_ctx.http.clone(), "ALERTE ROUGE, SANCRY JOUE À LOL. N'hésitez pas à lui dire d'aller se faire mettre en MP.").await;
			
			let sancry = GuildId::member(self.guild_id, _ctx.http.clone(), self.sancry_id).await;
			let sancry = sancry.unwrap();
			for _ in 1..10 {
				if let Err(_) = sancry.user.direct_message(_ctx.http.clone(), |m| {
					m.content("WTF SANCRY ARRÊTE DE JOUER À CE JEU DE CON TOUT DE SUITE")
				}).await
				{
					error!("Error sending message to that cunt");
				}
			}
		}
		else {
			let games: String = _new_data.activities.iter()
				.map(|x| format!("{} / ", x.name))
				.collect();
			info!("Sancry is playing {games}");
		}

		// let user = &_new_data.user;
		// let username = user.name.clone().unwrap_or("???".into());
		// for activity in _new_data.activities {
			// info!("{username} ({}) is playing {}", user.id, activity.name);
		// }
	}
}

// Permissions integer: 50565957942343

#[shuttle_runtime::main]
async fn serenity(
	#[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
	// Get the discord token set in `Secrets.toml`
	let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
		token
	} else {
		return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
	};
	let guild_id = if let Some(guild_id) = secret_store.get("GUILD_ID") {
		guild_id
	} else {
		return Err(anyhow!("'GUILD_ID' was not found").into());
	};
	let guild_id = GuildId(guild_id.parse().unwrap());
	let sancry_id = if let Some(sancry_id) = secret_store.get("SANCRY_ID") {
		sancry_id
	} else {
		return Err(anyhow!("'SANCRY_ID' was not found").into());
	};
	let sancry_id = sancry_id.parse().unwrap();

	// Set gateway intents, which decides what events the bot will be notified about
	let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT
		| GatewayIntents::GUILD_PRESENCES;

	let client = Client::builder(&token, intents)
		.event_handler(Bot{guild_id, sancry_id})
		.await
		.expect("Err creating client");

	Ok(client.into())
}
