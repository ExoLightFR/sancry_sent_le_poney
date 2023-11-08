#![allow(non_snake_case)]

use std::collections::{VecDeque, HashMap};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use serenity::client::Cache;
use serenity::http::Http;
use serenity::http::ratelimiting::RatelimitInfo;
use serenity::model::prelude::{Interaction, InteractionResponseType, Presence, Member};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use sqlx::{PgPool, Executor, FromRow};
use tokio::task::JoinHandle;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use shuttle_persist::PersistInstance;

mod songs;
mod bigbro;
mod rename;
mod abuse;

struct Handler;

pub struct BotData
{
	guild_id: GuildId,
	sancry_id: u64,
	is_singing: AtomicBool,
	singing_thread: RwLock<Option<JoinHandle<()>>>,
	cache: Arc<Cache>,
	http: Arc<Http>,
	songs_queue: RwLock<VecDeque<&'static str>>,
	aboos: RwLock<abuse::MuteData>,
	db: PgPool,
}

impl TypeMapKey for BotData {
	type Value = Arc<BotData>;
}

pub struct GuildData
{
	guild_id: GuildId,
	target_id: u64,
	singers: RwLock<HashMap<u64, VecDeque<&'static str>>>,
	singing_thread: RwLock<HashMap<u64, JoinHandle<()>>>,
}


#[derive(FromRow)]
pub struct GuildDataORM {
	id: i64,
	target_id: i64,
	fart_target: i64,
}

// impl GuildData {
// 	type SongQueue = RwLock<VecDeque<&'static str>>;
// }

// Get a read lock from ctx.data, and return the arc to the BotData struct in the TypeMap
pub async fn get_bot_data(ctx: &Context) -> Arc<BotData> {
	let data_read = ctx.data.read().await;
	return data_read.get::<BotData>().expect("fuck").clone();
}

impl BotData {
	pub fn new(guild_id: GuildId, sancry_id: u64, token: &str, db: PgPool) -> Self {
		BotData {
			guild_id,
			sancry_id,
			is_singing: AtomicBool::new(false),
			singing_thread: RwLock::new(None),
			cache: Arc::<Cache>::new(Cache::new()),
			http: Http::new(token).into(),	// Alternative syntax
			songs_queue: VecDeque::from([]).into(),
			aboos: abuse::MuteData::default().into(),
			db,
		}
	}

	async fn get_sancry(&self) -> Result<Member, SerenityError> {
		GuildId::member(self.guild_id, self.http.clone(), self.sancry_id).await
	}
}

async fn db_test(ctx: &Context, ready: &Ready) -> Result<(), Box<dyn Error>>
{
	let bot_data = get_bot_data(ctx).await;
	let guilds: Vec<GuildDataORM> = 
		sqlx::query_as("SELECT id, target_id, fart_target FROM GuildData")
			.fetch_all(&bot_data.db)
			.await?;
	for guild in &guilds {
		info!("ID: {}, tgt: {},  fart: {}", guild.id, guild.target_id, guild.fart_target);
	}

	for guild in &ready.guilds {
		info!("### ID: {}", guild.id);
	}
	return Ok(());
}

#[async_trait]
impl EventHandler for Handler {
	async fn ratelimit(&self, data: RatelimitInfo) {
		warn!("Rate limit hit: {:?}", data);
	}
	
	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);
		
		db_test(&ctx, &ready).await;
		
		let bot_data = get_bot_data(&ctx).await;

		GuildId::set_application_commands(&bot_data.guild_id, &ctx.http, |commands| {
			commands
			.create_application_command(|cmd| { cmd.name("hello").description("Se présente") })
			// .create_application_command(|cmd| songs::register_cmd(cmd))
			.create_application_command(|cmd| { cmd.name("tg").description("Ta gueule!") })
			// .create_application_command(|cmd| rename::register_cmd(cmd))
		}).await.unwrap();
	}

	async fn message(&self, ctx: Context, msg: Message) {
		let bot_data = get_bot_data(&ctx).await;
		bigbro::big_brother_is_watching(&bot_data, &ctx, &msg).await;
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		// check if the interaction is a command
		if let Interaction::ApplicationCommand(command) = interaction {

			let bot_data = {
				let data_read = ctx.data.read().await;
				data_read.get::<BotData>().expect("fuck").clone()
			};
 
			let response_content = match command.data.name.as_str() {
				"hello" => Ok("Salut. Ma première mission étant accomplie, je me reconvertis dans le faire-chiage de grande ampleur. À suivre.".to_string()),
				// "chante" => songs::exec_start_singing(&bot_data, &ctx, &command).await,
				"tg" => songs::exec_stop_singing(&bot_data, &command).await,
				// "rename" => rename::watashi_no_namae_ha_sankuri_desu(&ctx, &command).await,
				command => unreachable!("Unknown command: {}", command),
			};
			let response_content = match response_content {
				Ok(x) => x,
				Err(e) => { error!("{e}"); e },
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
		let bot_data = {
			let data_read = ctx.data.read().await;
			data_read.get::<BotData>().expect("fuck").clone()
		};
		if let Err(e) = bigbro::check_sancry_jeu_de_con(&bot_data, &ctx, &new_data).await {
			error!("Error checking is Sancry's playing LoL: {e}");
		}
	}
}

// Permissions integer: 50565957942343

#[shuttle_runtime::main]
async fn serenity(
	#[shuttle_secrets::Secrets] secret_store: SecretStore,
	// #[shuttle_persist::Persist] persist: PersistInstance,
	#[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_serenity::ShuttleSerenity {
	// Get the discord token set in `Secrets.toml`
	let token = secret_store.get("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found");

	let guild_id = secret_store.get("GUILD_ID").expect("'GUILD_ID' was not found");
	let guild_id = GuildId(guild_id.parse().unwrap());
	
	let sancry_id = secret_store.get("SANCRY_ID").expect("'SANCRY_ID' was not found");
	let sancry_id: u64 = sancry_id.parse().unwrap();

	// Set gateway intents, which decides what events the bot will be notified about
	let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT
		| GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS;

	pool.execute(include_str!("../schema.sql"))
		.await
		.map_err(|e| shuttle_runtime::Error::Database(format!("Migration error: {e}")))?;

	let client = Client::builder(&token, intents)
		.event_handler(Handler{})
		.await
		.expect("Err creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<BotData>(
			Arc::new(
				BotData::new(guild_id, sancry_id, token.as_str(), pool)
			)
		);
	}

	Ok(client.into())
}
