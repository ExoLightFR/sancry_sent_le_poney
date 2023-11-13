use std::time::SystemTime;

use sqlx::{FromRow, types::{Decimal, BigDecimal}};

#[derive(FromRow)]
pub struct User {
	id:  u64,
	points: i32,
	join_sound: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Guild {
	pub guild_id: BigDecimal,
	pub sing_id: Option<Decimal>,
	pub fart_id: Option<Decimal>,
}

#[derive(FromRow)]
pub struct Pin {
	msg_id: u64,
	num_reactions: i32,
	last_react_activity: SystemTime,
}

#[derive(FromRow)]
pub struct Mute {
	id: u64,
	reason: String,
}
