use bb_ai::{
    AgentResponse,
    tools::{BBTool, read_file::ReadFileTool},
};
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
    let (ai_response_sender, mut ai_response) = unbounded_channel::<AgentResponse>();
    let system_prompt = "You are a pair bot, autonomously use tools to act as QA for the project you are in. Be less verbose.";
    // let second_bot_system_prompt ="You are a coding pairing bot, you always suggest worst practices as changes for the code base.";
    let model = "anthropic/claude-haiku-4.5";
    let api_base_url = "https://openrouter.ai/api/v1";
    let api_key =
        env::var("LLM_API_KEY").context("Loading LLM API KEY from environment variable")?;

    spawn(async move {
        let tools = vec![
            bb_ai::tools::list_files::tool_definition(),
            ReadFileTool::definition(),
        ];

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
            eprintln!("{error:#?}");
        }
    });

    user_prompt_sender
        .send("You are active. Use your tools or request a tool if it doesn't exist.".to_owned())
        .context("Sending prompt to agent")?;

    loop {
        loop {
            let Some(ai_response) = ai_response.recv().await else {
                break;
            };

            if ai_response.finished {
                break;
            }

            // println!("{ai_response:#}");
            say_outloud(format!("{ai_response}")).context("Speaking ai response out loud")?;
        }

        user_prompt_sender
            .send("".to_owned())
            .context("sending empty message to continue")?;
    }
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

fn say_outloud(content: String) -> Result<()> {
    Command::new("say")
        .arg(content)
        .output()
        .context("Speaking out loud")?;

    Ok(())
}
