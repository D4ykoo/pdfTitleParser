mod extractor;
mod store;

fn main() {
    let path = "files/pdf-sample.pdf";

    let raw_title = extractor::extract_title(path).unwrap();
    let title = extractor::parse_title(&raw_title);
    println!("{}", title);

    store::save_pdf(path, &format!("files/renamed/{}.pdf", title)).unwrap();
}