use once_cell::sync::Lazy;

pub static COMMANDS_LIST: Lazy<Vec<String>> = Lazy::new(|| vec![String::from("init"), String::from("build")]);
