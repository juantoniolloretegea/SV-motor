use serde_json::json;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let bytes = std::fs::read(&args[1])?;
    let doc = lopdf::Document::load_mem(&bytes)?;
    let pages: Vec<_> = doc.get_pages().keys().map(|n| {
        doc.extract_text(&[*n]).map_err(|e|e.to_string())
    }).collect();
    let extract = pdf_extract::extract_text_from_mem_by_pages(&bytes)?;
    std::fs::write(&args[2], serde_json::to_vec_pretty(&json!({"lopdf_045":pages,"pdf_extract_0121":extract}))?)?;
    println!("Páginas: {}", doc.get_pages().len());
    Ok(())
}
