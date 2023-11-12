use serenity::{model::prelude::application_command::ApplicationCommandInteraction, json::Value};

pub fn flatten_cmd_data_option(
	command: &ApplicationCommandInteraction,
	index: usize
) -> Option<&Value>
{
	match command.data.options.get(index) {
		Some(arg) => match &arg.value {
			Some(value) => Some(value),
			None => None,
		},
		None => None,
	}
}
