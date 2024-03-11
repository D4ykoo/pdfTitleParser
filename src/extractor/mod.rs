use pdf::error::PdfError;
use pdf::file::{File, FileOptions};
use pdf::object::{FieldType, Resolve};


pub fn extract_title(file_path: &str) -> Result<String, PdfError>{
    let file = FileOptions::uncached().open(&file_path).unwrap();

    if let Some(ref info) = file.trailer.info_dict {
        return match &info.title {
            Some(title) => Ok(title.to_string()?),
            None => Err(PdfError::from("No title found".to_string())),
        }
    }
    Err(PdfError::from("No info dictionary found".to_string()))
}

pub fn parse_title(title: &str) -> String{
    let mut title = title.to_string();
    
    title.retain(|c| !r#"(),".;:'"#.contains(c));
    
    title = title.replace("\n", " ");
    title = title.replace("\r", " ");
    title = title.replace("\t", " ");
    title = title.replace("  ", " ");
    title = title.replace(" ", "_");

    title = title.trim().to_lowercase().to_string();

    title
}