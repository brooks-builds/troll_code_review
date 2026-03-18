use std::io::{Write, stdin, stdout};

use eyre::Context;

pub fn get_prompt() -> eyre::Result<String> {
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
