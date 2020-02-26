use dotenv::dotenv;

use std::{
    collections::HashMap,
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    model::{
        event::ResumedEvent,
        gateway::Ready,
    },
    prelude::*,
};

use log::{
    error,
    info,
};

mod commands;
use commands::dice::*;
use commands::general::*;
use commands::spell::*;

group!({
    name: "spelltome",
    options: {},
    commands: [get_spell, search_spells]
});
group!({
    name: "dice",
    options: {},
    commands: [roll]
});

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

struct SharedData;

impl TypeMapKey for SharedData {
    type Value = HashMap<String, String>;
}

fn main() {
    dotenv().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let prefix = match env::var("DISCORD_PREFIX") {
        Ok(some) => some,
        Err(_e) => String::from("~"),
    };
    let delay_: i64 = match env::var("DISCORD_DELAY") {
        Ok(some) => some.parse().unwrap(),
        Err(_e) => 2,
    };
    let time_span_: i64 = match env::var("DISCORD_TIME_SPAN") {
        Ok(some) => some.parse().unwrap(),
        Err(_e) => 30,
    };
    let limit_: i32 = match env::var("DISCORD_LIMIT") {
        Ok(some) => some.parse().unwrap(),
        Err(_e) => 3,
    };

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    {
        let mut data = client.data.write();
        let server =
            env::var("BACKEND_SERVER").expect("Expected the BACKEND_SERVER in the environment");
        let mut value: HashMap<String, String> = HashMap::new();
        value.insert("BACKEND_SERVER".to_string(), server);
        data.insert::<SharedData>(value);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(&prefix))
            .bucket("basic", |b| {
                b.delay(delay_).time_span(time_span_).limit(limit_)
            })
            .group(&SPELLTOME_GROUP)
            .group(&DICE_GROUP)
            .help(&MY_HELP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
