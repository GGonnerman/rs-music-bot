use std::sync::Arc;

use poise::{Framework, FrameworkBuilder, serenity_prelude as serenity};

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub fn framework_builder() -> FrameworkBuilder<Data, Error> {
    Framework::builder()
        .setup(|_, _, _| {
            Box::pin(async move {
                // construct user data here (invoked when bot connects to Discord)
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
                // command2(),
                // You can also modify a command by changing the fields of its Command instance
            ],
            ..Default::default()
        })
}

#[poise::command(slash_command, prefix_command, aliases("j", "abomination"))]
pub async fn join(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    Ok(())
}
