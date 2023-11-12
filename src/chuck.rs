use std::collections::HashMap;

use serenity::json::Value;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType, prelude::Context};

pub fn get_guides() -> HashMap< &'static str, HashMap<&'static str, &'static str> > {
	let planes = HashMap::from([
		("A-10C",		"https://chucksguides.com/aircraft/dcs/a-10c/"),
		("AV-8B",		"https://chucksguides.com/aircraft/dcs/av-8b/"),
		("F-14",		"https://chucksguides.com/aircraft/dcs/f-14b/"),
		("F-16CM",		"https://chucksguides.com/aircraft/dcs/f-16cm/"),
		("F/A-18C",		"https://chucksguides.com/aircraft/dcs/fa-18c/"),
		("JF-17",		"https://chucksguides.com/aircraft/dcs/jf-17/"),
		("M-2000C",		"https://chucksguides.com/aircraft/dcs/m-2000c/"),

		("AJS-37",		"https://chucksguides.com/aircraft/dcs/ajs-37/"),
		("F-5E",		"https://chucksguides.com/aircraft/dcs/f-5e3/"),
		("F-86F",		"https://chucksguides.com/aircraft/dcs/f-86f/"),
		("MiG-15bis",	"https://chucksguides.com/aircraft/dcs/mig-15bis/"),
		("MiF-19P",		"https://chucksguides.com/aircraft/dcs/mig-19p/"),
		("MiG-21bis",	"https://chucksguides.com/aircraft/dcs/mig-21bis/"),
		("Mirage F1",	"https://chucksguides.com/aircraft/dcs/mirage-f1/"),

		("C-101CC",		"https://chucksguides.com/aircraft/dcs/c-101cc/"),
		("L-39ZA",		"https://chucksguides.com/aircraft/dcs/l-39za/"),
		("Yak-52",		"https://chucksguides.com/aircraft/dcs/yak-52/"),
	]);

	let warbirds = HashMap::from([
		("Bf 109 K-4",			"https://chucksguides.com/aircraft/dcs/bf109k-4/"),
		("Fw190-A8",			"https://chucksguides.com/aircraft/dcs/fw190-a8/"),
		("Fw190-D9",			"https://chucksguides.com/aircraft/dcs/fw190-d9/"),
		("DH.98 Mosquito",		"https://chucksguides.com/aircraft/dcs/dh98/"),
		("P-47D",				"https://chucksguides.com/aircraft/dcs/p-47d/"),
		("P-51D",				"https://chucksguides.com/aircraft/dcs/p-51d/"),
		("Spitfire LF Mk.IX",	"https://chucksguides.com/aircraft/dcs/spitfire-lf-mk-ix/"),
		("I-16",				"https://chucksguides.com/aircraft/dcs/i-16/"),
	]);

	let helicopters = HashMap::from([
		("AH-64D",			"https://chucksguides.com/aircraft/dcs/ah-64d/"),
		("Ka-50",			"https://chucksguides.com/aircraft/dcs/ka-50/"),
		("Mi-8MTV2",		"https://chucksguides.com/aircraft/dcs/mi-8/"),
		("SA-342",			"https://chucksguides.com/aircraft/dcs/sa-342/"),
		("UH-1H",			"https://chucksguides.com/aircraft/dcs/uh-1h/"),
	]);

	return HashMap::from([
		("avion",		planes),
		("hélicoptère",	helicopters),
		("warbird",		warbirds),
	]);
}

pub fn register_cmd(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	let guides = get_guides();	
	
	cmd.name("chuck").description("RTFM");
	cmd.create_option(|opt| {
		opt.name("guide")
			.description("nique ta race")
			.kind(CommandOptionType::SubCommandGroup);
	
		for (category, modules) in guides {
			opt.create_sub_option(|subcmd| {
				subcmd.name(category)
					.description(format!("Obtenir le Chuck's Guide d'un {category}").as_str())
					.kind(CommandOptionType::SubCommand)
					.create_sub_option(|subopt| {
						subopt.name("module")
							.description("Le module DCS en question")
							.kind(CommandOptionType::String)
							.required(true);
						modules.keys().for_each(|name| { subopt.add_string_choice(name, name); });
						subopt
					})
			});
		}
		opt
	});
	cmd
}

/*
CommandDataOption {
	name: "guide",
	value: None,
	kind: SubCommandGroup,
	options: [
		CommandDataOption {
			name: "hélicoptère",
			value: None,
			kind: SubCommand,
			options: [
				CommandDataOption {
					name: "module",
					value: Some(String("UH-1H")),
					kind: String,
					options: [],
					resolved: Some(String("UH-1H")),
					focused: false
				}
			],
			resolved: None,
			focused: false
		}
	],
	resolved: None,
	focused: false
}
 */

fn extract_module(command: &ApplicationCommandInteraction) -> Option<(&String, &String)> {
	let category = &command.data.options.get(0)?.options.get(0)?.name;
	let module = &command.data.options.get(0)?.options.get(0)?.options.get(0)?.value;
	let module = match module {
		Some(Value::String(s)) => s,
		_ => return None,
	};
	return Some((category, module));
}

pub fn exec_chuck_cmd(_ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String, String> {

	let (category, module) = match extract_module(command) {
		Some((cat, module)) => (cat, module),
		None => return Err("Invalid or missing command argument".into()),
	};

	let guides = get_guides();
	let cat_guides	= guides.get(category.as_str()).ok_or("Catégorie de modules invalide".to_string())?;
	let guide		= cat_guides.get(module.as_str()).ok_or("Module invalide".to_string())?;

	return Ok(format!("Le Chuck's Guide du {module} se trouve ici : **<{guide}>**").into());
}
