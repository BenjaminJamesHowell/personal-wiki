use rocket::fs::{relative, NamedFile};
use serde::Serialize;
use serde_json::to_string;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

#[derive(Serialize)]
#[serde(tag = "status")]
enum ApiResponse {
    Ok { body: ApiContent },
    Err,
}

#[derive(Serialize)]
#[serde(tag = "requestType")]
enum ApiContent {
    PageList { title: String, pages: Vec<String> },
    Page { title: String, content: String },
}

#[get("/")]
pub fn index() -> String {
    match read_dir("./pages") {
        Err(_) => {
            return to_string(&ApiResponse::Err).unwrap();
        }

        Ok(pages) => {
            let pages = pages
                .map(|p| {
                    return p.unwrap().file_name().to_str().unwrap().to_owned();
                })
                .collect::<Vec<String>>();
            let response = ApiResponse::Ok {
                body: (ApiContent::PageList {
                    pages,
                    title: "page_list".to_owned(),
                }),
            };

            return to_string(&response).unwrap();
        }
    }
}

#[get("/<page>")]
pub async fn pages(page: &str) -> String {
    let path = String::from("./pages/") + page;
    let path = Path::new(&path);

    match read_to_string(path) {
        Err(_) => {
            return to_string(&ApiResponse::Err).unwrap();
        }

        Ok(content) => {
            let response = ApiResponse::Ok {
                body: ApiContent::Page {
                    title: page.to_owned(),
                    content,
                },
            };

            return to_string(&response).unwrap();
        }
    }
}

#[catch(404)]
pub async fn not_found() -> Option<NamedFile> {
    let path = Path::new(relative!("static/index.html"));

    return NamedFile::open(path).await.ok();
}
