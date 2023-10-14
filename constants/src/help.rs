use colored::Colorize;
use once_cell::sync::Lazy;

pub static HELP_COMMAND: Lazy<String> = Lazy::new(|| {
    format!(
        r#"{} {}

  {}    {}            Deploy the latest version of your project with Infralink.

  {}                        Setup a new projeect to deploy with Infralink.


  {}    {}            Build a docker image given a directory. 

  {}                     Get the latest version of the Infra CLI
  infra --help                Show all supported flags and commands

Learn more about Infralink: {}
Join our Discord community: {}"#,
        format!(
            "{} - deploy your app at any scale.",
            "Infralink".bright_magenta()
        ),
        format!("({})", env!("CARGO_PKG_VERSION")).bright_black(),
        "deploy".bright_magenta(),
        "./path".bright_black(),
        "init".bright_cyan(),
        "build".bright_blue(),
        "./path".bright_black(),
        "upgrade".bright_yellow(),
        "https://infralink.io/docs".bright_cyan(),
        "https://infralink.io/discord".bright_blue(),
    )
});
