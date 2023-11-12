use std::collections::HashMap;

use serenity::json::Value;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType, prelude::Context};

use crate::cmd_utils::flatten_cmd_data_option;

pub fn get_guides() -> HashMap<&'static str, &'static str> {
	HashMap::from([
		("A-10C",			"https://chucksguides.com/aircraft/dcs/a-10c/"),
		("AV-8B",			"https://chucksguides.com/aircraft/dcs/av-8b/"),
		("F-14",			"https://chucksguides.com/aircraft/dcs/f-14b/"),
		("F-16CM",			"https://chucksguides.com/aircraft/dcs/f-16cm/"),
		("F/A-18C",			"https://chucksguides.com/aircraft/dcs/fa-18c/"),
		("JF-17",			"https://chucksguides.com/aircraft/dcs/jf-17/"),
		("M-2000C",			"https://chucksguides.com/aircraft/dcs/m-2000c/"),

		("AJS-37",			"https://chucksguides.com/aircraft/dcs/ajs-37/"),
		("F-5E",			"https://chucksguides.com/aircraft/dcs/f-5e3/"),
		("F-86F",			"https://chucksguides.com/aircraft/dcs/f-86f/"),
		("MiG-15bis",		"https://chucksguides.com/aircraft/dcs/mig-15bis/"),
		("MiF-19P",			"https://chucksguides.com/aircraft/dcs/mig-19p/"),
		("MiG-21bis",		"https://chucksguides.com/aircraft/dcs/mig-21bis/"),
		("Mirage F1",		"https://chucksguides.com/aircraft/dcs/mirage-f1/"),

		("AH-64D",			"https://chucksguides.com/aircraft/dcs/ah-64d/"),
		("Ka-50",			"https://chucksguides.com/aircraft/dcs/ka-50/"),
		("Mi-8MTV2",		"https://chucksguides.com/aircraft/dcs/mi-8/"),
		("SA-342",			"https://chucksguides.com/aircraft/dcs/sa-342/"),
		("UH-1H",			"https://chucksguides.com/aircraft/dcs/uh-1h/"),

		("Bf 109 K-4",			"https://chucksguides.com/aircraft/dcs/bf109k-4/"),
		("Fw190-A8",			"https://chucksguides.com/aircraft/dcs/fw190-a8/"),
		("Fw190-D9",			"https://chucksguides.com/aircraft/dcs/fw190-d9/"),
		("DH.98 Mosquito",		"https://chucksguides.com/aircraft/dcs/dh98/"),
		("P-47D",				"https://chucksguides.com/aircraft/dcs/p-47d/"),
		("P-51D",				"https://chucksguides.com/aircraft/dcs/p-51d/"),
		("Spitfire LF Mk.IX",	"https://chucksguides.com/aircraft/dcs/spitfire-lf-mk-ix/"),
		("I-16",				"https://chucksguides.com/aircraft/dcs/i-16/"),

		("C-101CC",		"https://chucksguides.com/aircraft/dcs/c-101cc/"),
		("L-39ZA",		"https://chucksguides.com/aircraft/dcs/l-39za/"),
		("Yak-52",		"https://chucksguides.com/aircraft/dcs/yak-52/"),
	])
}

pub fn register_cmd(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd.name("chuck").description("RTFM")
		.create_option(|opt| {
			opt.name("module")
				.description("Le Chuck's Guide du module")
				.kind(CommandOptionType::String)
				.required(true);
			get_guides().keys().for_each(|module| { opt.add_string_choice(module, module); });
			opt
		})
}

pub fn exec_chuck_cmd(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String, String> {
	let module = match flatten_cmd_data_option(command, 0) {
		Some(Value::String(s)) => s,
		_ => return Err("Missing or invalid command argument".to_string()),
	};
	let guides = get_guides();
	let url = guides.get(module.as_str());
	let url = match url { Some(x) => x, None => return Err("Module invalide!".into()) };
	
	return Ok(format!("Chuck's Guide du {module} : **<{url}>**"));
}
