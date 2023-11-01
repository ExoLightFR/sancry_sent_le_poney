#![allow(non_snake_case)]

use std::error::Error;
use std::sync::atomic::{AtomicBool};

use anyhow::anyhow;
use serenity::model::prelude::{Interaction, InteractionResponseType, Presence, ActivityType};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tokio::task::JoinHandle;
use tokio::sync::RwLock;
use tracing::{error, info};

mod songs;
mod bigbro;

struct Bot
{
	guild_id: GuildId,
	sancry_id: u64,
	is_singing: AtomicBool,
	singing_thread: RwLock<Option<JoinHandle<()>>>,
}

impl Default for Bot {
	fn default() -> Self {
		Bot {
			guild_id: GuildId(0),
			sancry_id: 0,
			is_singing: AtomicBool::new(false),
			singing_thread: RwLock::new(None),
		}
	}
}

impl Bot {
	async fn check_sancry_LoL(&self, ctx: &Context, data: &Presence) -> Result<(), Box<dyn Error>> {
		if data.user.id != self.sancry_id {
			return Ok(());
		}
		let mut activities = data.activities.iter()
			.filter(|x| x.kind == ActivityType::Playing || x.kind == ActivityType::Competing);
		if activities.any(|x| x.name == "League of Legends") {
			info!("ATTENTION!!! SANCRY JOUE A LOL");
			let sancry = GuildId::member(self.guild_id, ctx.http.clone(), self.sancry_id)
				.await?;
			for _ in 1..10 {
				sancry.user.direct_message(ctx.http.clone(), |m| {
					m.content("WTF SANCRY ARRÊTE DE JOUER À CE JEU DE CON TOUT DE SUITE")
				}).await?;
			}
		}
		return Ok(());
	}
}

/*
Cannot respond to slash command:
Http(
	UnsuccessfulRequest(
		ErrorResponse {
			status_code: 404,
			url: Url {
				scheme: "https",
				cannot_be_a_base: false,
				username: "",
				password: None,
				host: Some(Domain("discord.com")),
				port: None,
				path: "/api/v10/interactions/1169399345003630693/aW50ZXJhY3Rpb246MTE2OTM5OTM0NTAwMzYzMDY5MzpxMTJuQTZDbEtZOUo2b0hLMGQ4ZjVVNlA5YUU0bExhUXhzaFJQdTUyN28zclkxalpjbklxZjk2eG1QZVF6R002MEVxbUtWUGhYb3c4UXFFajV6c2VoOG1PcnpjM0RIdkFNMXowbTRsaWxMcnNBTTNER09lY0packNFSk82VThsbg/callback",
				query: None,
				fragment: None
			},
			error: DiscordJsonError {
				code: 10062,
				message: "Unknown interaction",
				errors: []
			}
		}
	)
)
*/

enum ResponseKind {
	Instant(String),
	Delayed(String),
}

#[async_trait]
impl EventHandler for Bot {
	async fn message(&self, ctx: Context, msg: Message) {
		bigbro::big_brother_is_watching(&self, &ctx, &msg).await;
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
			commands
				.create_application_command(|cmd| { cmd.name("hello").description("Se présente") })
				.create_application_command(|cmd| songs::register_songs_command(cmd) )
				.create_application_command(|cmd| { cmd.name("tg").description("Ta gueule!") })
		}).await.unwrap();
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		// check if the interaction is a command
		if let Interaction::ApplicationCommand(command) = interaction {
 
			let response_content =
				match command.data.name.as_str() {
					"hello" => ResponseKind::Instant("Salut. Je suis un bot créé dans le seul et unique but de faire chier Sancry. À suivre.".into()),
					"chante" => ResponseKind::Instant(songs::fetch_song(&self, &ctx, &command.data.options[0].value).await),
					"tg" => songs::exec_stop_singing(&self, &ctx, &command).await,
					command => unreachable!("Unknown command: {}", command),
				};
			// send `response_content` to the discord server
			match response_content {
				ResponseKind::Instant(response_content) => {
					command.create_interaction_response(&ctx.http, |response| {
						response
							.kind(InteractionResponseType::ChannelMessageWithSource)
							.interaction_response_data(|message| message.content(response_content))
					})
						.await.expect("Cannot respond to slash command");
				}
				ResponseKind::Delayed(response_content) => {
					// command.create_followup_message(&ctx.http, |response| {
					// 	response
					// 		.kind(InteractionResponseType::ChannelMessageWithSource)
					// 		.interaction_response_data(|message| message.content(response_content))
					// }).await.expect("fuck");
					command.create_followup_message(&ctx.http, |response| {
						response.content(response_content)
					}).await.expect("yolo");
				},
			};
			info!("Aaaaaand we're done");
		}
	}

	async fn presence_update(&self, ctx: Context, new_data: Presence) {
		if let Err(e) = self.check_sancry_LoL(&ctx, &new_data).await {
			error!("Error checking is Sancry's playing LoL: {e}");
		}
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
		| GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS;

	let client = Client::builder(&token, intents)
		.event_handler(Bot{guild_id, sancry_id, ..Default::default()})
		.await
		.expect("Err creating client");

	Ok(client.into())
}
