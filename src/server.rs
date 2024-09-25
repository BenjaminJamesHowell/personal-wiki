use crate::html::{self, PageHTML};
use rocket::http::ContentType;

#[get("/")]
pub fn redir_to_home_from_root() -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::home_redirect().unwrap_or(html::err_404().expect("Cannot load 404 file")),
    );
}

#[get("/pages")]
pub fn redir_to_home_from_pages() -> (ContentType, String) {
    return (
        ContentType::HTML,
        html::home_redirect().unwrap_or(html::err_404().expect("Cannot load 404 file")),
    );
}

#[get("/pages/<title>")]
pub fn pages(mut title: &str) -> (ContentType, String) {
    if title.ends_with(".html") {
        title = title.get(..(title.len() - 5)).unwrap();
    }

    return (
        ContentType::HTML,
        html::page(title)
            .map(|PageHTML { output, fm: _ }| output)
            .unwrap_or(html::err_404().expect("Cannot load 404 file")),
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
pub fn category(mut title: &str) -> (ContentType, String) {
    if title.ends_with(".html") {
        title = title.get(..(title.len() - 5)).unwrap();
    }

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
