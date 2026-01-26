use std::env;
use std::sync::Arc;

use poise::{Framework, FrameworkBuilder, serenity_prelude as serenity};
use poise::builtins::register_in_guild;

use tracing::error;


pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub fn framework_builder() -> FrameworkBuilder<Data, Error> {
    Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // construct user data here (invoked when bot connects to Discord)
                let guild_id_str = env::var("GUILD_ID");
                match guild_id_str {
                    Ok(id_str) => {
                        let guild_id: u64 = id_str.parse().map_err(|e| {
                            error!("Failed to parse GUILD_ID: {}", e);
                            e
                        })?;
                        register_in_guild(ctx, &framework.options().commands, serenity::GuildId::new(guild_id)).await?;
                    }
                    Err(_) => {
                        error!("GUILD_ID not found in environment, skipping guild command registration");
                    }
                }
                Ok(Data {})
            })
        })
        // Most configuration is done via the `FrameworkOptions` struct, which you can define with
        // a struct literal (hint: use `..Default::default()` to fill uninitialized
        // settings with their default value):
        .options(poise::FrameworkOptions {
            // on_error: |err| Box::pin(my_error_function(err)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                // edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                //     std::time::Duration::from_secs(3600),
                // ))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            // This is also where commands go
            commands: vec![
                join(),
                aboot(),
                // command2(),
                // You can also modify a command by changing the fields of its Command instance
            ],
            ..Default::default()
        })
}


// !join, !leave, !play - !p, !queue <depth> - !q <depth>, !nowplaying - !np, !skip <n>, !clear,  !shuffle - !s,  !insert <index1>,
// !move <index1> <index2>, !history <depth>, !pomodoro, !search <name>, !select <result index> <optional queue position; default=end of list>, !help


#[poise::command(slash_command, prefix_command, aliases("_"))]
pub async fn __(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    // 
    // 
    Ok(())
}


#[poise::command(slash_command, prefix_command, aliases("j"))]
pub async fn join(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    // if owner in vc join voice channel of the owner
    // respond and log that the bot has joined the voice channel
    Ok(())
}


#[poise::command(slash_command, prefix_command, aliases("l"))]
pub async fn leave(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    // if owner in vc leave voice channel of the owner
    // ?respond? and log that the bot has left the voice channel
    Ok(())
}

#[poise::command(slash_command, prefix_command, aliases("p"))]
pub async fn play(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    // check the user is in voice channel
    // determine if valid url
    // respond and log
    // initiate download
    // start playing music
    Ok(())
}

#[poise::command(slash_command, prefix_command, aliases("q"))]
pub async fn queue(ctx: Context<'_>, depth: String) -> Result<(), Error> {
    // respond and log
    Ok(())
}

#[poise::command(slash_command, prefix_command, aliases("np"))]
pub async fn nowplaying(ctx: Context<'_>) -> Result<(), Error> {
    // respond and log
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>, n: String) -> Result<(), Error> {
    // check the user is in voice channel
    // bounds check
    // skip n number or songs 
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    // check the user is in voice channel
    // clear out the queue
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), Error> {
    // check the user is in voice channel
    // shuffle the queue
    Ok(())
}


#[poise::command(slash_command, prefix_command, aliases("about"))]
pub async fn aboot(ctx: Context<'_>, boot: String) -> Result<(), Error> {
    let response = format!("In canada we about all aboot {boot}");
    ctx.say(response).await?;

    Ok(())
}