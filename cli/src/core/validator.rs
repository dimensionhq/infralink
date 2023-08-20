use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone)]
pub enum ValidatedOptions {
    Init {},
    Help {},
    None,
}

pub fn validate(
    command: &str,
    _options: LinkedHashMap<String, Option<String>>,
    _raw_args: Option<Vec<String>>,
) -> ValidatedOptions {
    match command {
        "init" => ValidatedOptions::Init {},
        "help" => ValidatedOptions::Help {},
        _ => ValidatedOptions::None,
    }
}
