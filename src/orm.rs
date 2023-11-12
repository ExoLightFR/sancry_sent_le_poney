#[derive(FromRow)]
pub struct User {
	id:  u64,
	points: i32,
	join_sound: Option<String>,
}

#[derive(FromRow)]
pub struct Guild {
	id: u64,
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
