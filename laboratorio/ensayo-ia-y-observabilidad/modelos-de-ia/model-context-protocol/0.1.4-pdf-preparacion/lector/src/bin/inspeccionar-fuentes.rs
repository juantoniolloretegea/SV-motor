fn main()->Result<(),Box<dyn std::error::Error>> {
 let a:Vec<_>=std::env::args().collect();let d=lopdf::Document::load(&a[1])?;
 if a.len()>2 {for id in [58,59,65] {let raw=d.get_object((id,0))?.as_stream()?.decompressed_content()?;println!("Objeto {id}: {}",String::from_utf8_lossy(&raw));}return Ok(());}
 for (i,p) in d.get_pages(){
  if i<9 {continue;}
  for (name,font) in d.get_page_fonts(p)? {
   println!("Página {i}, recurso {:?}, fuente {:?}",String::from_utf8_lossy(&name),font);
   if let Ok(o)=font.get(b"Encoding").and_then(|o|o.as_reference()) {println!("Codificación: {:?}",d.get_object(o)?);}
  }
 }
 Ok(())
}
