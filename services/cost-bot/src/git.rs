use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks};
use walkdir::WalkDir;

pub fn clone(repository_name: String, _credentials: String, mut builder: RepoBuilder) {
    let callbacks = RemoteCallbacks::new();
    let mut options = FetchOptions::new();

    options.download_tags(git2::AutotagOption::None);
    options.update_fetchhead(false);
    options.depth(1);

    options.remote_callbacks(callbacks);

    builder.fetch_options(options);

    builder
        .clone(
            &format!("https://github.com/{}.git", repository_name),
            Path::new(&format!("./{}", repository_name.split('/').last().unwrap())),
        )
        .unwrap();
}

pub fn configuration_files(repository_name: String) -> HashMap<PathBuf, String> {
    let mut files = HashMap::new();

    // Search for all infra.toml files recursively using the walkdir crate
    let walker = WalkDir::new(Path::new(&format!(
        "./{}",
        repository_name.split('/').last().unwrap()
    )));

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let contents = std::fs::read_to_string(path).unwrap();

            files.insert(path.to_path_buf(), contents);
        }
    }

    files
}
