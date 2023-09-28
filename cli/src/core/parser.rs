use super::validator;
use super::validator::ValidatedOptions;
use constants::commands::COMMANDS_LIST;
use linked_hash_map::LinkedHashMap;
use miette::Result;

use std::env;

// Parse command-line arguments passed in
pub async fn parse() -> Result<ValidatedOptions> {
    // Collect command-line arguments passed in
    let args = env::args().collect::<Vec<String>>();

    // The command to be executed
    let command: &str;

    // Identify the command to be executed
    if args.len() > 1 {
        if COMMANDS_LIST.contains(&args[1]) {
            command = &args[1];
        } else {
            // Invalid command
            command = "none";
        }
    } else {
        // If no arguments are passed in, default to `help`
        // ie: when running `infra`
        command = "help";
    }

    // We want to store all flags and their values in order using a LinkedHashMap
    let mut options: LinkedHashMap<String, Option<String>> = LinkedHashMap::new();

    let mut skip_next = false;

    // Parse all flags into the options LinkedHashMap
    for (index, arg) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if arg.starts_with('-') {
            // Remove all leading "-"
            let flag = arg.trim_start_matches('-');

            // Check if the next arg exists
            if index + 1 < args.len() {
                let next_arg = args[index + 1].clone();

                // Ensure it isn't a flag
                if !next_arg.starts_with('-') {
                    options.insert(flag.to_string(), Some(next_arg));

                    // Skip the next iteration
                    skip_next = true;
                } else {
                    options.insert(flag.to_string(), None);
                }
            } else {
                options.insert(flag.to_string(), None);
            }
        } else if index > 1 {
            options.insert(arg.to_string(), None);
        }
    }

    // Raw argument string passed in, this might be required by some commands
    // We skip the first argument as it is the path to the executable
    let raw_args: Vec<String> = args
        .iter()
        .skip(1)
        .map(|i| i.to_owned())
        .collect::<Vec<String>>();

    // Validate the command and the options generated
    Ok(validator::validate(
        command,
        options.clone(),
        Some(raw_args),
    ))
}
