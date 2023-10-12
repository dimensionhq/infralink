use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use git2::{build::RepoBuilder, FetchOptions, RemoteCallbacks};
use indexmap::IndexMap;
use types::config::InfrastructureConfiguration;
use walkdir::WalkDir;

use crate::cost;

pub fn clone(
    repository_name: String,
    _credentials: String,
    specific_ref: String,
    mut builder: RepoBuilder,
) {
    let callbacks = RemoteCallbacks::new();
    let mut options = FetchOptions::new();

    options.download_tags(git2::AutotagOption::None);
    options.update_fetchhead(false);
    options.depth(1);

    options.remote_callbacks(callbacks);

    builder.fetch_options(options);

    // Construct the URL and local path
    let url = format!("https://github.com/{}.git", repository_name);
    let local_path = format!("./{}", repository_name.split('/').last().unwrap());

    // Clone the repository to a specific path
    let repo = builder.clone(&url, Path::new(&local_path)).unwrap();

    // Checkout to the specific ref
    let object = repo.revparse_single(&specific_ref).unwrap();
    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force();
    repo.checkout_tree(&object, Some(&mut checkout_opts))
        .unwrap();

    // Set HEAD to point to our specific ref
    repo.set_head(&format!("refs/heads/{}", specific_ref))
        .unwrap();
}

pub fn delete(repository_name: String) {
    std::fs::remove_dir_all(Path::new(&format!(
        "./{}",
        repository_name.split('/').last().unwrap()
    )))
    .unwrap();
}

pub fn configuration_files(repository_name: String) -> IndexMap<PathBuf, String> {
    let mut files = IndexMap::new();

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

    let mut file_vec: Vec<(PathBuf, String)> = files.into_iter().collect();

    // Sort the vector based on the depth of the file tree
    file_vec.sort_by(|a, b| {
        let depth_a = a.0.components().count();
        let depth_b = b.0.components().count();
        depth_a.cmp(&depth_b)
    });

    files = file_vec.into_iter().collect();

    files
}

pub async fn cost_breakdowns(
    files: IndexMap<PathBuf, String>,
) -> IndexMap<String, IndexMap<String, f64>> {
    let mut breakdowns: IndexMap<String, IndexMap<String, f64>> = IndexMap::new();

    for (_path, contents) in files {
        let config = InfrastructureConfiguration::from_str(&contents).unwrap();

        // next, parse the contents of the infra.toml file and analyse it.
        let breakdown = cost::calculate_cost(&config).await;

        breakdowns.insert(config.app.name, breakdown);
    }

    breakdowns
}
