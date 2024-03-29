use actix_web::{HttpRequest, web, Result};
use actix_files as fs;
use actix_web::http::StatusCode;

pub fn js1(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    return Ok(fs::NamedFile::open(format!("static/js/{}", path.0))?) ;
}

pub fn js2(req: HttpRequest, path: web::Path<(String, String)>) -> Result<fs::NamedFile> {
    return Ok(fs::NamedFile::open(format!("static/js/{}/{}", path.0, path.1))?) ;
}

pub fn img1(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    return Ok(fs::NamedFile::open(format!("static/img/{}", path.0))?) ;
}

pub fn img2(req: HttpRequest, path: web::Path<(String, String)>) -> Result<fs::NamedFile> {
    return Ok(fs::NamedFile::open(format!("static/img/{}/{}", path.0, path.1))?) ;   
}

pub fn css(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("static/css/{}", path.0))?)
}

pub fn fonts(req: HttpRequest, path: web::Path<(String,)>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("static/fonts/{}", path.0))?)
}

/// 404 handler
pub fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/views/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}