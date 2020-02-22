use log::error;
use reqwest;
use serenity::framework::standard::{
    macros::command,
    Args,
    CommandError,
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use splitterrust_db::models::spell_schools::Spell as SpellSchools;
use std::env;

#[command]
#[bucket = "basic"]
pub fn get_spell(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    // TODO get this only once and pass it to the functions
    //      each call to env would be expensive
    let server =
        env::var("BACKEND_SERVER").expect("Expected the BACKEND_SERVER in the environment");
    let url = format!("{}/spell/{}", server, name);
    println!("{}", url);

    match reqwest::get(&url) {
        Ok(mut result) => {
            let json: SpellSchools = match result.json() {
                Ok(j) => j,
                Err(e) => {
                    error!("Error retrieving json: {:?}", e);
                    return Err(CommandError::from(e));
                }
            };

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
