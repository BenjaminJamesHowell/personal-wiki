use crate::html;
use rocket::http::ContentType;

#[get("/pages/<title>")]
pub fn pages(title: &str) -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::page(title).unwrap_or(html::err_404().expect("Cannot load 404 file")),
    );
}

#[get("/search")]
pub fn search() -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::search().unwrap_or(html::err_404().expect("Cannot load 404 file")),
    );
}

#[get("/categories/<title>")]
pub fn category(title: &str) -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::category(title).unwrap_or(html::err_404().expect("Cannot load 404 file")),
    );
}

#[catch(404)]
pub fn err_404() -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::err_404().expect("Cannot load 404 file"),
    );
}
