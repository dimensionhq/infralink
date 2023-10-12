use constants::help::HELP_COMMAND;
use miette::Result;

pub async fn execute() -> Result<()> {
    println!("{}", HELP_COMMAND.as_str());

    Ok(())
}
