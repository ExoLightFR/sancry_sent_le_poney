use std::{collections::HashMap, sync::{atomic::Ordering, Arc}, time::Duration, error::Error};
use serenity::{builder::CreateApplicationCommand, model::prelude::{command::CommandOptionType, Member, application_command::ApplicationCommandInteraction}, prelude::Context, json::Value};
use tracing::{info, error};

use crate::{BotData, get_bot_data};

// use crate::Bot;

static CONNEMARA: &str = "Terre brûlée au vent
Des landes de pierres
Autour des lacs, c'est pour les vivants
Un peu d'enfer, le Connemara
Des nuages noirs qui viennent du nord
Colorent la terre, les lacs, les rivières
C'est le décor du Connemara
Au printemps suivant, le ciel irlandais était en paix
Maureen a plongé nue dans un lac du Connemara
Sean Kelly s'est dit \"je suis catholique\", Maureen aussi
L'église en granit de Limerick, Maureen a dit \"oui\"
De Tipperary, Barry-Connelly et de Galway
Ils sont arrivés dans le comté du Connemara
Y avait les Connors, les O'Connolly, les Flaherty du Ring of Kerry
Et de quoi boire trois jours et deux nuits
Là-bas, au Connemara
On sait tout le prix du silence
Là-bas, au Connemara
On dit que la vie, c'est une folie
Et que la folie, ça se danse
Terre brûlée au vent
Des landes de pierres
Autour des lacs, c'est pour les vivants
Un peu d'enfer, le Connemara
Des nuages noirs qui viennent du nord
Colorent la terre, les lacs, les rivières
C'est le décor du Connemara
On y vit encore au temps des Gaëls et de Cromwell
Au rythme des pluies et du soleil
Aux pas des chevaux
On y croit encore aux monstres des lacs
Qu'on voit nager certains soirs d'été
Et replonger pour l'éternité
On y voit encore
Des hommes d'ailleurs venus chercher
Le repos de l'âme et pour le cœur, un goût de meilleur
L'on y croit encore
Que le jour viendra, il est tout près
Où les Irlandais feront la paix autour de la Croix
Là-bas, au Connemara
On sait tout le prix de la guerre
Là-bas, au Connemara
On n'accepte pas
La paix des Gallois
Ni celle des rois d'Angleterre";

static JE_VEUX_CHANTER: &str = "J'ai chanté en province et chanté aux terrasses
Sur les couverts qui grincent et sur le bruit des tasses
Les bals du samedi soir où personne ne m'écoute
C'est comme un grand espoir qui fout le camp goutte à goutte
On m'a dit quelquefois que j'avais du talent
Mais pour miser sur moi on n'avait pas le temps
Je ne suis qu'un artiste, la doublure d'un génie
Tous comme un trapéziste dont on ne voit que la vie
Je veux chanter
J'en ai assez d'être dans l'ombre d'un artiste
J'en ai assez d'être une star dans les coulisses
Je veux chanter, je veux chanter
Je veux chanter
Avec mon nom, avec ma voix, je veux chanter
Et ma voix s'appelle comme moi, je veux chanter
Regardez-moi, écoutez-moi, j'existe
J'ai tant rêvé que ce soit moi qu'on applaudisse
J'ai fait tellement de disques que je ne peux plus les compter
Mais derrière les artistes juste pour les aider
J'ai d'autres ambitions et j'ai d'autres talents
Je veux passer le pont, je veux chanter devant
Je veux être une idole à présent, il est temps
Je veux des soirées folles, des chapiteaux géants
Je veux voir ma photo sur les murs de vos villes
Être dans vos sonos et dans vos magazines
Je veux chanter
J'en ai assez d'être dans l'ombre d'un artiste
J'en ai assez d'être une star dans les coulisses
Je veux chanter, je veux chanter
Je veux chanter
Avec mon nom, avec ma voix, je veux chanter
Et ma voix s'appelle comme moi, je veux chanter
Regardez-moi, écoutez-moi, j'existe
J'ai tant rêvé que ce soit moi qu'on applaudisse
Je veux chanter
J'en ai assez d'être dans l'ombre d'un artiste
J'en ai assez d'être une star dans les coulisses
Je veux chanter, je veux chanter
Je veux chanter
Avec mon nom, avec ma voix, je veux chanter
Et ma voix s'appelle comme moi, je veux chanter
Regardez-moi, écoutez-moi, j'existe
J'ai tant rêvé que ce soit moi qu'on applaudisse";

static LES_DEMONS_DE_MINUIT: &str = "Rue déserte, dernière cigarette
Plus rien ne bouge
Juste un bar qui éclaire le trottoir
D'un néon rouge
J'ai besoin de trouver quelqu'un
J'veux pas dormir
Je cherche un peu de chaleur
À mettre dans mon cœur
Ils m'entraînent au bout de la nuit
Les démons de minuit
M'entraînent jusqu'à l'insomnie
Les fantômes de l'ennui
Dans mon verre, je regarde la mer
Qui se balance (qui se balance)
J'veux un disque de funky music
Faut que ça danse (faut que ça danse)
J'aime cette fille sur talons-aiguilles
Qui se déhanche
Ça met un peu de chaleur
Au fond de mon cœur
Ils m'entraînent au bout de la nuit
Les démons de minuit
M'entraînent jusqu'à l'insomnie
Les fantômes de l'ennui
Ils m'entraînent au bout de la nuit
Les démons de minuit
M'entraînent jusqu'à l'insomnie
Les fantômes de l'ennui
J'aime cette fille sur talons-aiguilles
Qui se déhanche
Ça met un peu de chaleur
Au fond de mon cœur
Ils m'entraînent au bout de la nuit (jusqu'au bout de la nuit)
Les démons de minuit
M'entraînent jusqu'à l'insomnie (ils m'entraînent)
Les fantômes de l'ennui (ooh-ooh-ooh)
Ils m'entraînent au bout de la nuit";

static ALEXANDRIE_ALEXANDRA: &str = "Voile sur les filles, barques sur le Nil
Je suis dans ta vie, je suis dans tes bras
Alexandra, Alexandrie
Alexandrie, où l'amour danse avec la nuit
J'ai plus d'appétit qu'un barracuda
Je boirai tout le Nil si tu me reviens pas
Je boirai tout le Nil si tu me retiens pas
Alexandrie, Alexandra
Alexandrie, où l'amour danse au fond des bras
Ce soir j'ai de la fièvre et toi, tu meurs de froid
Les sirènes du port d'Alexandrie
Chantent encore la même mélodie
La lumière du phare d'Alexandrie
Fait naufrager les papillons de ma jeunesse
Voile sur les filles et barques sur le Nil
Je suis dans ta vie, je suis dans tes draps
Alexandra, Alexandrie
Alexandrie, où tout commence et tout finit
J'ai plus d'appétit qu'un barracuda
Je te mangerai crue si tu me reviens pas
Je te mangerai crue si tu me retiens pas
Alexandrie, Alexandra
Alexandrie, ce soir je danse dans tes draps
Je te mangerai crue si tu me retiens pas
Les sirènes du port d'Alexandrie
Chantent encore la même mélodie
La lumière du phare d'Alexandrie
Fait naufrager les papillons de ma jeunesse
Rah! Ha! Rah! Ha!
Voile sur les filles, barques sur le Nil
Alexandrie, Alexandra
Ce soir j'ai de la fièvre et toi, tu meurs de froid
Ce soir je danse, je danse, je danse dans tes bras
Allez danse! Oui, danse!
Danse, danse, danse, danse!
Alexandrie, Alexandra
Ce soir je danse, je danse, je dans dans tes bras";

static EVERY_TIME_WE_TOUCH: &str = "I still hear your voice when you sleep next to me
I still feel your touch in my dreams
Forgive me my weakness, but I don't know why
Without you, it's hard to survive
'Cause every time we touch
I get this feeling
And every time we kiss
I swear I could fly
Can't you feel my heart beat fast
I want this to last
Need you by my side
'Cause every time we touch
I feel this static
And every time we kiss
I reach for the sky
Can't you hear my heart beat so
I can't let you go
Want you in my life
Your arms are my castle
Your heart is my sky
They wipe away tears that I cry
The good and the bad times
We've been through them all
You make me rise when I fall
'Cause every time we touch
I get this feeling
And every time we kiss
I swear I can fly
Can't you feel my heart beat fast
I want this to last
Need you by my side
'Cause every time we touch
I feel this static
And every time we kiss
I reach for the sky
Can't you hear my heart beat so
I can't let you go
Want you in my life
'Cause every time we touch
I get this feeling
And every time we kiss
I swear I could fly
Can't you feel my heart beat fast
I want this to last
Need you by my side";

static TEST_1: &'static str = "quel gros con";
static TEST_2: &'static str = "coucou les amis";

pub fn get_songs() -> HashMap<&'static str, &'static str> {
	let songs = HashMap::from([
		("Les Lacs du Connemara",	CONNEMARA),
		("Je veux chanter",			JE_VEUX_CHANTER),
		("Les démons de minuit",	LES_DEMONS_DE_MINUIT),
		("Alexandrie Alexandra",	ALEXANDRIE_ALEXANDRA),
		("Every time we touch",		EVERY_TIME_WE_TOUCH),
		("Test 1",					TEST_1),
		("Test 2",					TEST_2),
	]);
	return songs;
}

pub fn register_cmd(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	cmd.name("chante").description("Fait chanter Sancry.")
		.create_option(|opt| {
			opt.name("chanson")
				.description("Le titre à fair chanter à Sancry")
				.kind(CommandOptionType::String)
				.required(true);
			get_songs().iter().for_each(|(name, _)| { opt.add_string_choice(name, name); } );
			return opt;
		})
}

pub async fn exec_start_singing(
	bot: &Arc<BotData>,
	ctx: &Context,
	command: &ApplicationCommandInteraction
) -> Result<String, String>
{
	let song_choice = match command.data.options.get(0) {
		Some(arg) => match &arg.value {
			Some(Value::String(choice)) => choice,
			_ => return Err("Error: missing or invalid argument".to_string()),
		},
		None => return Err("Error: invalid argument".to_string()),
	};

	info!("{} chose {song_choice}", command.user.name);
	let songs = get_songs();
	let song = match songs.get(song_choice.as_str()) {
		Some(&song) => song,
		None => return Err("Impossible de trouver la chanson".into()),
	};

	{
		let bot = get_bot_data(ctx).await;
		let mut queue = bot.songs_queue.write().await;
		if queue.len() < 5 {
			queue.push_back(song);
		}
		else {
			return Err("Trop de chansons en attente!".into())
		}
	}

	let sancry_id = bot.sancry_id;
	if !bot.is_singing.load(Ordering::Relaxed) {
		let ctx2 = ctx.clone();
		let sancry = match bot.get_sancry().await {
			Ok(x) => x,
			Err(e) => return Err(e.to_string()),
		};
		
		let mut handle = bot.singing_thread.write().await;
		bot.is_singing.swap(true, Ordering::Relaxed);
		*handle = Some(tokio::spawn(async move {
			let _ = noubliez_pas_les_paroles(ctx2.clone(), sancry).await;
			let bot_data = get_bot_data(&ctx2).await;
			bot_data.is_singing.swap(false, Ordering::Relaxed);
		}));
	}
	Ok(format!("C'est parti pour la musique! <@{}> va chanter \"{song_choice}\"", sancry_id))
}

pub async fn noubliez_pas_les_paroles(ctx: Context, sancry: Member) -> Result <(), Box<dyn Error>> {
	loop {
		let song: String;
		{
			let bot = get_bot_data(&ctx).await;
			let mut queue = bot.songs_queue.write().await;
			if queue.is_empty() {
				break;
			}
			song = queue.pop_front().unwrap().to_uppercase();
		}
		let song_words: Vec<&str> = song.split_ascii_whitespace().collect();
		for word in song_words {
			info!("{word}");
			sancry.edit(ctx.http.clone(), |m| m.nickname(word)).await?;
			tokio::time::sleep(Duration::from_secs(10)).await;
		}
	}
	return Ok(());
}

pub async fn exec_stop_singing(bot: &Arc<BotData>, command: &ApplicationCommandInteraction) -> Result<String, String> {
	if command.user.id == bot.sancry_id {
		return Ok("mdr t'as cru".into());
	}
	if let Err(e) = bot.get_sancry().await.unwrap().edit(bot.http.clone(), |x| x.nickname("")).await {
		error!("Failed to change Sancry's name: {e}");
	}
	let response: String = match il_a_oublié_les_paroles(bot).await {
		true => "Allez ça suffit, tg Sancry",
		false => "Mais enfin, il ne chante pas !",
	}.into();
	return Ok(response);
}

pub async fn il_a_oublié_les_paroles(bot_data: &Arc<BotData>) -> bool
{
	if bot_data.is_singing.load(Ordering::Relaxed) {
		info!("Ta gueule Sancry");
		let thread = bot_data.singing_thread.read().await;
		let thread = thread.as_ref().unwrap();
		thread.abort();
		bot_data.is_singing.swap(false, Ordering::Relaxed);
		return true;
	}
	else {
		info!("Sancry a déjà fermé sa gueule");
		return false;
	}
}
