use log::{
    debug,
    error,
    info,
};
use percent_encoding::{
    utf8_percent_encode,
    AsciiSet,
    CONTROLS,
};
use reqwest::Url;
use serenity::framework::standard::{
    macros::command,
    Args,
    CommandError,
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use splitterrust_db::models::spell_schools::Spell as SpellSchools;

use crate::SharedData;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'%');

#[command]
#[bucket = "basic"]
pub fn search_spells(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    if name.len() > 15 {
        info!("Search string was longer than 15: {}", &name);
        let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.content(
                "Spell name to long. If you think this is false-positive contact the owner of \
                 this bot.",
            );
            m
        });
        if let Err(why) = msg {
            error!("Error sending message: {:?}", why);
            return Err(CommandError::from(why));
        }
        return Ok(());
    }

    let data = ctx.data.read();
    let shared_data = data.get::<SharedData>().unwrap();
    let server = match shared_data.get("BACKEND_SERVER") {
        Some(s) => s,
        None => return Err(CommandError::from("Failed to connec to server")),
    };
    let url_ = utf8_percent_encode(&format!("{}/spell/{}", server, name), FRAGMENT).to_string();
    let url = Url::parse(&url_)?;

    match reqwest::get(url) {
        Ok(mut result) => {
            let json: Vec<SpellSchools> = match result.json() {
                Ok(j) => j,
                Err(e) => {
                    error!("Error retrieving json: {:?}", e);
                    let msg = msg.channel_id.send_message(&ctx.http, |m| {
                        m.content(
                            "Sorry, I failed to find spells with that name :face_with_monocle:",
                        );
                        m
                    });
                    if let Err(why) = msg {
                        error!("Error sending message: {:?}", why);
                        return Err(CommandError::from(why));
                    }
                    return Ok(());
                }
            };

            let mut spell_fields = Vec::new();
            for spell in &json {
                spell_fields.push((&spell.name, &spell.effect, false))
            }

            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Here are your spells");
                m.embed(|e| {
                    e.title(&name);
                    e.description("Results from query:");
                    e.fields(spell_fields);
                    e
                });
                m
            });

            if let Err(why) = msg {
                error!("Error sending message: {:?}", why);
                return Err(CommandError::from(why));
            }
            Ok(())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(CommandError::from(e))
        }
    }
}

#[command]
#[bucket = "basic"]
pub fn get_spell(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    if name.len() > 15 || name.contains('%') {
        info!("Search string was longer than 15 or contains %: {}", &name);
        let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.content(
                "Spell name to long or invalid characters detected. If you think this is \
                 false-positive contact the owner of this bot.",
            );
            m
        });
        if let Err(why) = msg {
            error!("Error sending message: {:?}", why);
            return Err(CommandError::from(why));
        }
        return Ok(());
    }

    let data = ctx.data.read();
    let shared_data = data.get::<SharedData>().unwrap();
    let server = match shared_data.get("BACKEND_SERVER") {
        Some(s) => s,
        None => return Err(CommandError::from("Failed to connec to server")),
    };
    let url = Url::parse(&format!("{}/spell/{}", server, name))?;
    debug!("{}", &url);

    match reqwest::get(url) {
        Ok(mut result) => {
            let json_vec: Vec<SpellSchools> = match result.json() {
                Ok(j) => j,
                Err(e) => {
                    error!("Error retrieving json: {:?}", e);
                    let msg = msg.channel_id.send_message(&ctx.http, |m| {
                        m.content("Sorry, I failed to find that spell :face_with_monocle:");
                        m
                    });
                    if let Err(why) = msg {
                        error!("Error sending message: {:?}", why);
                        return Err(CommandError::from(why));
                    }
                    return Ok(());
                }
            };

            let json = &json_vec[0];

            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Here is your Spell");
                m.embed(|e| {
                    e.title(&json.name);
                    e.description(&json.effect);
                    e.fields(vec![
                        ("Kosten:", &json.cost, false),
                        ("Dauer:", &json.cast_duration, false),
                        //("Optionen", &json.options, false),
                        ("Reichweite:", &json.range, false),
                        ("Schwierigkeit:", &json.difficulty, false),
                        ("Typus:", &json.typus, false),
                        ("Verstärkt:", &json.enforced, false),
                        ("Effekt:", &json.effect, false),
                        ("Wirkungsdauer:", &json.duration_of_effect, false),
                        ("Optionen:", &format!("{:?}", &json.options), false),
                    ]);
                    e
                });
                m
            });

            if let Err(why) = msg {
                error!("Error sending message: {:?}", why);
                return Err(CommandError::from(why));
            }
            Ok(())
        }
        Err(e) => {
            error!("{:?}", e);
            Err(CommandError::from(e))
        }
    }
}
