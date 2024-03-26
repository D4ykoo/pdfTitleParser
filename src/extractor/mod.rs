use log::error;
use pdf::error::PdfError;
use pdf::file::FileOptions;

pub fn extract_title(file_path: &str) -> Result<String, PdfError> {
    if !file_path.ends_with(".pdf") {
        error!("Not a pdf file: {}", file_path);
        return Err(PdfError::from("Not a pdf file".to_string()));
    }

    if !std::path::Path::new(file_path).exists() {
        // error!("File not found: {}", file_path);
        return Ok("".to_string());
    }

    let res = FileOptions::uncached().open(file_path);

    let file = match res {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening file: {}", e);
            return Ok(file_path.to_string());
        }
    };

    if let Some(ref info) = file.trailer.info_dict {
        return match &info.title {
            Some(title) => {
                if title.to_string().unwrap().is_empty() {
                    Ok(file_path.to_string())
                } else {
                    Ok(title.to_string().unwrap())
                }
            }
            None => {
                error!("No title found");
                Err(PdfError::from("No title found".to_string()))
            }
        };
    }
    error!("No info dictionary found");
    Err(PdfError::from("No info dictionary found".to_string()))
}

pub fn parse_title(title: &str) -> String {
    let mut title = title.to_string();

    title.retain(|c| !r#"(),".;:'"#.contains(c));

    title = title.replace('\n', " ");
    title = title.replace('\r', " ");
    title = title.replace('\t', " ");
    title = title.replace("  ", " ");
    title = title.replace(' ', "_");
    title = title.replace('-', "_");

    title = title.trim().to_lowercase().to_string();

    title
}
