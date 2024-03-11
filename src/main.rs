mod extractor;
mod store;

use std::path::Path;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

fn event_cb(res: Result<notify::Event>){
    match res {
        Ok(event) => {
            if event.kind.is_create() {
                let path = event.paths[0].to_str().unwrap();
                let raw_title = extractor::extract_title(path).unwrap();
                let title = extractor::parse_title(&raw_title);
                store::save_pdf(path, &format!("files/renamed/{}.pdf", title)).unwrap();
            }
        },
        Err(e) => println!{"watch error: {:?}", e}
    }
}

fn main() {
    loop {
        let mut watcher = notify::recommended_watcher(event_cb).unwrap();
        let _ = watcher.watch(Path::new("files/"), RecursiveMode::Recursive);
    }
}