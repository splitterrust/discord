use log::error;
use reqwest;
use serenity::framework::standard::{
    macros::command,
    Args,
    CommandError,
    CommandResult,
};
use serenity::http::AttachmentType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use splitterrust_db::models::spell_schools::Spell as SpellSchools;
use std::env;
use std::path::Path;

#[command]
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

#[command]
pub fn test(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Hello, World!");
        m.embed(|e| {
            e.title("This is a title");
            e.description("This is a description");
            // e.image("attachment://splittermond_logo.png");
            e.fields(vec![
                ("This is the first field", "This is a field body", true),
                (
                    "This is the second field",
                    "Both of these fields are inline",
                    true,
                ),
            ]);
            e.field(
                "This is the third field",
                "This is not an inline field",
                false,
            );
            e.footer(|f| {
                f.text("This is a footer");

                f
            });
            e.color((0, 255, 0));
            e.thumbnail("attachment://splittermond_logo.png");

            e
        });
        m.add_file(AttachmentType::Path(Path::new(
            "./src/static/splittermond_logo.png",
        )));
        m
    });
    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
        return Err(CommandError::from(why));
    }
    Ok(())
}
