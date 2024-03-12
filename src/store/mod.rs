use std::fs;

pub fn save_pdf(source: &str, destination: &str) {
    let _ = fs::rename(
        std::path::Path::new(source),
        std::path::Path::new(destination),
    );
}
