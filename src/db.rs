use std::error::Error;

use serenity::{prelude::Context, model::prelude::Ready};
use shuttle_persist::PersistInstance;
use sqlx::{FromRow, PgPool, Executor};
use tracing::*;

use crate::get_bot_data;



#[derive(FromRow)]
pub struct GuildDataORM {
	id: i64,
	target_id: Option<i64>,
	fart_target: Option<i64>,
}

#[derive(FromRow, Debug)]
struct Chokbar {
	id: i64,
	other_id: Option<i64>
}

// pub async fn db_test(ctx: &Context, ready: &Ready) -> Result<(), Box<dyn Error>>
// {
// 	let bot_data = get_bot_data(ctx).await;

// 	info!("In db_test");

// 	// sqlx::query("INSERT INTO GuildData (id, target_id) VALUES ($1, $2)")
// 	// 	.bind(1234)
// 	// 	.bind(5678)
// 	// 	.execute(&bot_data.db)
// 	// 	.await?;

// 	// info!("Did INSERT");
	
// 	// let guilds: Vec<GuildDataORM> = 
// 	// 	sqlx::query_as("SELECT id, target_id, fart_target FROM GuildData")
// 	// 		.fetch_all(&bot_data.db)
// 	// 		.await?;

// 	// info!("Did SELECT");

// 	// sqlx::query!("INSERT INTO chokbar (id, other_id) VALUES ($1, $2) ON CONFLICT(id) DO NOTHING")
// 	// 	.bind(1234)
// 	// 	.bind(5678)
// 	// 	.execute(&bot_data.db)
// 	// 	.await?;
// 	sqlx::query!("INSERT INTO chokbar (id, other_id) VALUES ($1, $2) ON CONFLICT(id) DO NOTHING", 1235, 5678)
// 		.execute(&bot_data.db)
// 		.await?;

// 	let guilds: Vec<Chokbar> = sqlx::query_as!(Chokbar, "SELECT * FROM chokbar")
// 		.fetch_all(&bot_data.db)
// 		.await?;

// 	guilds.iter().for_each(|x| info!("{:?}", x));
// 	// for guild in &guilds {
// 	// 	info!("ID: {}, tgt: {:?},  fart: {:?}", guild.id, guild.target_id, guild.fart_target);
// 	// }

// 	for guild in &ready.guilds {
// 		info!("### ID: {}", guild.id);
// 	}
// 	return Ok(());
// }

// Migrations list in persist map are in form {timestamp: String, file_name: String}
pub async fn my_migrate(persist: &PersistInstance, pool: &PgPool) -> Result<(), Box<dyn Error>> {
	// Load list of all finished migrations
	let finished_migrations_keys = persist.list()?;
	let finished_migrations: Vec<String> = finished_migrations_keys.iter()
		.map( |key| persist.load::<String>(key).expect("BLYAT") )
		.collect();
	
	// Get list of all migration files
	let files = std::fs::read_dir("./migrations")?;
	let mut file_names: Vec<String> = vec![];
	for file in files {
		let file = file?.file_name().into_string().expect("Fatal err with filesys");
		if !file.ends_with(".sql") {
			continue;
		}
		file_names.push(file);
	}

	// Remove all finished migrations from list and sort them by date
	file_names = file_names.into_iter()
		.filter(|x| !finished_migrations.contains(x))
		.collect();
	file_names.sort();

	// Execute all unexecuted migrations, and remember them in the Persist instance
	for file in file_names {
		info!("Trying to apply migration of `{file}'...");
		let path = format!("./migrations/{file}");
		let query = std::fs::read_to_string(path)?;
		pool.execute(query.as_str()).await?;

		let cut = file.find('_').unwrap_or(file.len());	// Get a slice of only the timestamp before the '_'
		let key = &file[..cut];
		persist.save(key, file.clone())?;
		info!("Successfully executed migration of `{file}'");
	}

	return Ok(());
}
