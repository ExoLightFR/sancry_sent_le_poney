use std::time::SystemTime;

use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
	pub user_id: String,
	pub guild_id: String,
	pub points: i32,
	pub join_sound: Option<String>,
	pub forced_name: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Guild {
	pub guild_id: String,
	pub sing_id: Option<String>,
	pub fart_id: Option<String>,
}

#[derive(FromRow)]
pub struct Pin {
	msg_id: String,
	num_reactions: i32,
	last_react_activity: SystemTime,
	guild_id: String,
	user_id: String,
}

#[derive(FromRow)]
pub struct Mute {
	id: u64,
	reason: String,
}
