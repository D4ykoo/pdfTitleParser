use std::fs;

pub fn save_pdf(source: &str, destination: &str) -> std::io::Result<()> {
    fs::rename(source, destination)?;
    Ok(())
}
