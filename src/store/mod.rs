use std::fs;

use log::debug;

pub fn save_pdf(source: &str, destination: &str) {
    let res = fs::rename(
        std::path::Path::new(source),
        std::path::Path::new(destination),
    );
    debug!("rename result: {:?}", res);
}
