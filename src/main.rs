use eyre::{Context, Result};
use std::{
    env,
    io::{Write, stdin, stdout},
    process::Command,
};
use tokio::{spawn, sync::mpsc::unbounded_channel};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let (user_prompt_sender, user_prompt_receiver) = unbounded_channel::<String>();
    let (ai_response_sender, mut ai_response) = unbounded_channel();
    let system_prompt = "You are a nice dev who is pairing on a project. Your job is to check the code base for quality, ask questions, and offer suggestions.";
    // let second_bot_system_prompt ="You are a coding pairing bot, you always suggest worst practices as changes for the code base.";
    let model = "anthropic/claude-haiku-4.5";
    let api_base_url = "https://openrouter.ai/api/v1";
    let api_key =
        env::var("LLM_API_KEY").context("Loading LLM API KEY from environment variable")?;

    spawn(async move {
        let tools = vec![bb_ai::tools::list_files::tool_definition()];

        if let Err(error) = bb_ai::run(
            user_prompt_receiver,
            ai_response_sender,
            system_prompt,
            model,
            api_base_url,
            api_key,
            tools,
        )
        .await
        {
            eprintln!("{error:?}");
        }
    });

    loop {
        let user_prompt = get_prompt()?;
        user_prompt_sender
            .send(user_prompt)
            .context("Sending prompt to agent")?;

        let Some(ai_response) = ai_response.recv().await else {
            break;
        };

        println!("{ai_response:#}");
        say_outloud(&ai_response).context("Speaking ai response out loud")?;
    }

    println!("Agent quit");

    Ok(())
}

fn get_prompt() -> Result<String> {
    let mut result = String::new();

    print!("> ");
    stdout()
        .flush()
        .context("flushing standard out to print prompt")?;

    stdin()
        .read_line(&mut result)
        .context("getting user prompt")?;

    Ok(result)
}

fn say_outloud(content: &str) -> Result<()> {
    Command::new("say")
        .arg(content)
        .spawn()
        .context("Speaking out loud")?;

    Ok(())
}
