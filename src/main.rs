mod extractor;
mod store;

use clap::Parser;
use log::{debug, error};
use notify::{RecursiveMode, Result, Watcher};
use serde::{Deserialize, Serialize};
use std::{env, path::Path, thread::sleep};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    source: String,
    target: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliInput {
    #[arg(short = 's', long = "sourcePath")]
    source_path: Option<std::path::PathBuf>,
    #[arg(short = 't', long = "targetPath")]
    target_path: Option<std::path::PathBuf>,
}

fn event_cb(res: Result<notify::Event>, target: &str) {
    match res {
        Ok(event) => {
            debug!("event: {:?}", event);
            if event.kind.is_create() || event.kind.is_modify() && !event.kind.is_remove() {
                let path_str = event.paths[0].to_str().unwrap();
                let mut path = path_str.to_string();

                if path_str.contains(".crdownload") {
                    sleep(std::time::Duration::from_secs(2));
                    path = path_str.replace(".crdownload", "");
                    debug!(".crdownload: {:?}", path);
                }

                if path_str.contains(".part") {
                    sleep(std::time::Duration::from_secs(2));
                    path = path_str.replace(".part", "");
                    debug!(".part: {:?}", path);
                }

                let raw_title = extractor::extract_title(&path).unwrap();
                let title = extractor::parse_title(&raw_title);

                store::save_pdf(&path, &format!("{}{}.pdf", target, title));
            }
        }
        Err(e) => {
            println! {"watch error: {:?}", e};
            error!("watch error: {:?}", e);
        }
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
    debug!("Config read: \n{:?}", &config);
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
            error!(
                "Root access required since the HOME environment variable is not set: {:?}",
                e
            )
        }
    }
    let config_dir_str = format!("{}/.config/pdfTitleParser/", home);
    let config_dir = std::path::Path::new(&config_dir_str);

    let config_file_str = format!("{}config.toml", config_dir_str);
    let config_file = std::path::Path::new(&config_file_str);
    (config_dir.to_path_buf(), config_file.to_path_buf())
}

fn update_config(config_file: &std::path::PathBuf, config: &Config) {
    let config = toml::to_string(&config).expect("Could not serialize config");
    std::fs::write(config_file, &config).expect("Could not write config file");
    debug!("Config updated: \n{}", &config);
}

fn main() {
    env_logger::init();

    let args = CliInput::parse();

    let (config_dir, config_file) = construct_config_paths();

    let mut default_source = String::from("");
    let mut default_target = String::from("");

    // if config file not present set default values
    if !config_file.exists() {
        debug!("Config file not found, setting default values");
        default_source.push_str("/home/pdfs/");
        default_target.push_str("/home/pdfs/");
    } else {
        debug!("Config file found, reading values");
        let config = read_config(&config_file);
        default_source = config.source;
        default_target = config.target;
    }

    let source = match args.source_path {
        Some(path) => path.to_string_lossy().into_owned(),
        None => default_source.to_string(),
    };

    let target = match args.target_path {
        Some(path) => path.to_string_lossy().into_owned(),
        None => default_target.to_string(),
    };

    // on first run, create config file
    init_config(&source, &target, &config_dir, &config_file);

    // update source and target of config file
    // if source or target are different from the config file
    // dont fall to default values
    let mut config = read_config(&config_file);
    if config.source != source || config.target != target {
        config.source = source.to_string();
        config.target = target.to_string();
        update_config(&config_file, &config);
    }

    let config = read_config(&config_file);

    loop {
        let target = config.target.clone();
        let mut watcher = notify::recommended_watcher(move |res| event_cb(res, &target)).unwrap();
        let _ = watcher.watch(Path::new(&config.source), RecursiveMode::Recursive);
    }
}
