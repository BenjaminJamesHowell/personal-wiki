use crate::page::PageFrontMatter;
use rocket::fs::{relative, NamedFile};
use serde::Serialize;
use serde_json::to_string;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

fn serialize(api_response: ApiResponse) -> Option<String> {
    match api_response {
        None => {
            return Some(format!(r#"{{"status":"Err"}}"#));
        }

        Some(body) => {
            let body_str = to_string(&body).ok()?;
            return Some(format!(r#"{{"status":"Ok","body":{}}}"#, body_str,));
        }
    };
}
type ApiResponse = Option<ApiContent>;

#[derive(Serialize)]
#[serde(tag = "requestType")]
enum ApiContent {
    PageList { title: String, pages: Vec<String> },
    Page { title: String, content: String },
    Category { title: String, pages: Vec<String> },
}

#[get("/")]
pub fn index() -> String {
    return serialize(handle_index()).unwrap();
}

fn handle_index() -> ApiResponse {
    let pages = read_dir("./pages")
        .ok()?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    return Some(ApiContent::PageList {
        title: "page_list".to_owned(),
        pages,
    });
}

#[get("/category/<title>")]
pub fn category(title: &str) -> String {
    return serialize(handle_category(title)).unwrap();
}

fn handle_category(title: &str) -> ApiResponse {
    let pages = read_dir("./pages").ok()?;
    let mut category_pages: Vec<String> = vec![];

    for file_path in pages {
        let file_path = file_path.ok()?;
        let file_path = file_path.path();
        let file_name = file_path.file_name()?;
        let file_path = file_path.as_os_str();

        let contents = read_to_string(&file_path).ok()?;

        // Extract the front matter as the content between the first two instances of "---"
        let parts: Vec<&str> = contents.split("---").collect();
        let fm = match parts.len() {
            0 => "",
            1 => "",
            _ => parts.get(1).unwrap(),
        };
        let fm: PageFrontMatter = serde_yaml::from_str(fm).ok()?;

        if fm.categories.contains(&title.to_string()) {
            category_pages.push(file_name.to_owned().into_string().ok()?);
        }
    }

    return Some(ApiContent::Category {
        title: title.to_owned() + "_(category)",
        pages: category_pages,
    });
}

#[get("/<page>")]
pub async fn pages(page: &str) -> String {
    return serialize(handle_page(page)).unwrap();
}

fn handle_page(page: &str) -> ApiResponse {
    let path = format!("./pages/{}", page);
    let path = Path::new(&path);

    let content = read_to_string(path).ok()?;

    return Some(ApiContent::Page {
        title: page.to_owned(),
        content,
    });
}

#[catch(404)]
pub async fn not_found() -> Option<NamedFile> {
    let path = Path::new(relative!("static/index.html"));

    return NamedFile::open(path).await.ok();
}
