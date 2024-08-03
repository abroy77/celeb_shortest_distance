use actix_files::NamedFile;
use std::path::PathBuf;

pub async fn homepage() -> actix_web::Result<NamedFile> {
    let path: PathBuf = ["static", "index.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}
