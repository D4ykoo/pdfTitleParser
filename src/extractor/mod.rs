use pdf::error::PdfError;
use pdf::file::FileOptions;

pub fn extract_title(file_path: &str) -> Result<String, PdfError> {
    if !file_path.ends_with(".pdf") {
        return Err(PdfError::from("Not a pdf file".to_string()));
    }

    if !std::path::Path::new(file_path).exists() {
        return Ok("".to_string());
    }

    let res = FileOptions::uncached().open(file_path);

    let file = match res {
        Ok(f) => f,
        Err(_) => return Ok(file_path.to_string()),
    };

    if let Some(ref info) = file.trailer.info_dict {
        return match &info.title {
            Some(title) => Ok(title.to_string()?),
            None => Err(PdfError::from("No title found".to_string())),
        };
    }
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
