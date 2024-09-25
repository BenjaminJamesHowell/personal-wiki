use rocket::fs::relative;
use std::fs::{read_dir, read_to_string};

use crate::page::{extract_front_matter, format_title, parse_md, InfoboxItem, PageFrontMatter};

#[derive(Debug)]
pub struct PageHTML {
    pub output: String,
    pub fm: PageFrontMatter,
}

pub fn page(title: &str) -> Option<PageHTML> {
    let base = index()?;
    let raw_contents = read_to_string(format!("./pages/{}", title)).ok()?;
    let contents = parse_md(&raw_contents)?;
    let display_title = format_title(title);

    let fm = extract_front_matter(&raw_contents)?;
    let categories = fm
        .categories
        .iter()
        .map(|category| {
            return format!(
                r#"<a href="/categories/{}.html">{}</a>"#,
                category,
                format_title(category),
            );
        })
        .collect::<String>();

    let properties = fm
        .infobox
        .iter()
        .map(|item| {
            let InfoboxItem { name, value } = item;

            return format!(
                r#"
<div class="property">
    <div class="property-name">{}</div>
    <div class="property-value">{}</div>
</div>
                "#,
                name, value,
            );
        })
        .collect::<String>();

    let mut infobox = String::new();
    if properties.len() != 0 {
        infobox += &format!(
            r#"<div id="info-box"><h2 class="title">{}</h2>"#,
            format_title(&title)
        );
        infobox += &properties;
        infobox += "</div>";
    }

    let output = base
        .replace("[CONTENT]", &contents)
        .replace("[TITLE]", &display_title)
        .replace("[CATEGORIES]", &categories)
        .replace("[INFOBOX]", &infobox);

    return Some(PageHTML { output, fm });
}

pub fn search() -> Option<String> {
    let pages = read_dir("./pages")
        .ok()?
        .map(|file_path| {
            return Some(file_path.ok()?.path().file_name()?.to_str()?.to_owned());
        })
        .collect::<Option<Vec<String>>>()?;

    return serde_json::to_string(&pages).ok();
}

pub fn category(title: &str) -> Option<String> {
    let pages = read_dir("./pages").ok()?;
    let pages = pages.map(|file_path| {
        let file_path = file_path.ok()?;
        let file_path = file_path.path();
        let file_name = file_path.file_name()?.to_str()?;
        let file_path = file_path.as_os_str();

        let contents = read_to_string(&file_path).ok()?;
        let fm = extract_front_matter(&contents)?;

        if !fm.categories.contains(&title.to_string()) {
            return None;
        }

        return Some(format!(
            r#"<li><a href="/pages/{}.html">{}</a></li>"#,
            file_name,
            format_title(file_name),
        ));
    });
    let mut pages_html = String::from("<ul>");
    for page in pages {
        if let Some(page) = page {
            pages_html += &page;
        }
    }
    pages_html += "</ul>";

    let display_title = format_title(title) + " (category)";
    return Some(
        index()?
            .replace("[CONTENT]", &pages_html)
            .replace("[TITLE]", &display_title)
            .replace("[CATEGORIES]", "")
            .replace("[INFOBOX]", ""),
    );
}

fn index() -> Option<String> {
    return read_to_string(relative!("./static/index.html")).ok();
}

pub fn err_404() -> Option<String> {
    return read_to_string(relative!("./static/404")).ok();
}

pub fn home_redirect() -> Option<String> {
    return read_to_string(relative!("./static/redirect-to-home.html")).ok();
}
