#![allow(non_snake_case)]

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use anyhow::anyhow;
use serenity::json::Value;
use serenity::model::prelude::{Interaction, InteractionResponseType, Presence, ActivityType};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod songs;

struct Bot
{
	guild_id: GuildId,
	sancry_id: u64,
	is_singing: AtomicBool,
	singing_thread: Option<tokio::task::JoinHandle<()>>,
}

impl Default for Bot {
	fn default() -> Self {
		Bot {
			guild_id: GuildId(0),
			sancry_id: 0,
			is_singing: AtomicBool::new(false),
			singing_thread: None,
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

	fn fetch_song(&self, ctx: &Context, command_option: &Option<Value>) -> String {
		let song_choice = match command_option {
			Some(Value::String(choice)) => choice,
			_ => "",
		};

		info!("song choice: [{song_choice}]");
		let songs = songs::get_songs();
		let result = match songs.get(&song_choice) {
			Some(song) => Ok(song.to_uppercase()),
			None => Err("Impossible de trouver la chanson"),
		};
		if result.is_err() {
			return result.unwrap_err().into();
		}

		let ctx2 = ctx.clone();
		let id = self.sancry_id;
		let guild_id = self.guild_id;
		if !self.is_singing.load(Ordering::Relaxed) {
			tokio::spawn(async move {
				songs::noubliez_pas_les_paroles(&ctx2.clone(), result.unwrap(),
					guild_id, id).await;
			});
			self.is_singing.swap(true, Ordering::Relaxed);
		}
		return format!("C'est parti pour la musique! <@{}> va chanter \"{song_choice}\"", self.sancry_id);
	}
}

#[async_trait]
impl EventHandler for Bot {
	async fn message(&self, _ctx: Context, msg: Message) {
		if msg.content.to_lowercase() == "sancry" {
			info!("foo");
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
			commands
				.create_application_command(|cmd| { cmd.name("hello").description("Se présente") })
				.create_application_command(|cmd| songs::register_songs_command(cmd) )
		}).await.unwrap();
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		// check if the interaction is a command
		if let Interaction::ApplicationCommand(command) = interaction {
 
			let response_content =
				match command.data.name.as_str() {
					"hello" => "Salut. Je suis un bot créé dans le seul et unique but de faire chier Sancry. À suivre.".to_string(),
					"chante" => self.fetch_song(&ctx, &command.data.options[0].value),
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

	async fn presence_update(&self, ctx: Context, new_data: Presence) {
		if let Err(e) = self.check_sancry_LoL(&ctx, &new_data).await {
			error!("Error checking is Sancry's playing LoL: {e}");
		}
		// let games: String = new_data.activities.iter()
		// 	.map(|x| format!("{} / ", x.name))
		// 	.collect();
		// info!("{:?} is playing {games}", new_data.user.name);
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
