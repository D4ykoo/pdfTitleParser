mod extractor;
mod store;

use notify::{RecursiveMode, Result, Watcher};
use serde::{Deserialize, Serialize};
use std::{env, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    source: String,
    target: String,
}

fn event_cb(res: Result<notify::Event>, target: &str) {
    match res {
        Ok(event) => {
            if event.kind.is_create() {
                let path = event.paths[0].to_str().unwrap();
                let raw_title = extractor::extract_title(path).unwrap();
                let title = extractor::parse_title(&raw_title);
                store::save_pdf(path, &format!("{}/{}.pdf", target, title)).unwrap();
            }
        }
        Err(e) => println! {"watch error: {:?}", e},
    }
}

fn init_config(
    source_dir: &str,
    target_dir: &str,
    config_dir: &std::path::PathBuf,
    config_file: &std::path::PathBuf,
) {
    // if config file does not exist, create it
    if !config_file.exists() {
        std::fs::create_dir_all(config_dir).expect("Could not create config directory");
        let config = Config {
            source: source_dir.to_string(),
            target: target_dir.to_string(),
        };
        let config = toml::to_string(&config).expect("Could not serialize config");
        std::fs::write(config_file, config).expect("Could not write config file");
    }
}

fn read_config(config_file: &std::path::PathBuf) -> Config {
    // if config file exists, read it
    let config = std::fs::read_to_string(config_file).expect("Could not read config file");
    let config: Config = toml::from_str(&config).expect("Could not parse config file");

    config
}

fn construct_config_paths() -> (std::path::PathBuf, std::path::PathBuf) {
    let mut home = String::from("");

    match env::var("HOME") {
        Ok(val) => home.push_str(&val),
        Err(e) => {
            home.push_str("/etc/");
            println!(
                "Root access required since the HOME environment variable is not set: {:?}",
                e
            );
        }
    }
    // create config dir
    let config_dir_str = format!("{}/.config/pdfTitleParser/", home);
    let config_dir = std::path::Path::new(&config_dir_str);

    // create config file
    let config_file_str = format!("{}config.toml", config_dir_str);
    let config_file = std::path::Path::new(&config_file_str);
    (config_dir.to_path_buf(), config_file.to_path_buf())
}

fn main() {
    let source = "/home/pdfs/";
    let target = "/home/pdfs/";

    let (config_dir, config_file) = construct_config_paths();

    // on first run, create config file
    init_config(source, target, &config_dir, &config_file);
    let config = read_config(&config_file);

    loop {
        let target = config.target.clone();
        let mut watcher = notify::recommended_watcher(move |res| event_cb(res, &target)).unwrap();
        let _ = watcher.watch(Path::new(&config.source), RecursiveMode::Recursive);
    }
}
