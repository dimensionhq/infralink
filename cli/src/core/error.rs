use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum CommandError {
    #[error("Command not found")]
    #[diagnostic(
        code(infra::cli::CommandNotFound),
        help("Run `infra help` to see a list of available commands")
    )]
    CommandNotFound,
}
