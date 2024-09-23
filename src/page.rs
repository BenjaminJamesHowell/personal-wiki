use markdown::to_html_with_options;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PageFrontMatter {
    #[serde(default)]
    pub categories: Vec<String>,

    #[serde(default)]
    pub infobox: Vec<InfoboxItem>,
}

#[derive(Deserialize, Debug)]
pub struct InfoboxItem {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub value: String,
}

pub fn parse_md(contents: &str) -> Option<String> {
    return to_html_with_options(
        contents,
        &markdown::Options {
            compile: markdown::CompileOptions {
                ..Default::default()
            },
            parse: markdown::ParseOptions {
                constructs: markdown::Constructs {
                    frontmatter: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        },
    )
    .ok();
}

pub fn extract_front_matter(contents: &str) -> Option<PageFrontMatter> {
    let parts: Vec<&str> = contents.split("---").collect();
    let fm = match parts.len() {
        0 => "",
        1 => "",
        _ => parts.get(1)?,
    };
    return serde_yaml::from_str(fm).ok();
}

pub fn format_title(title: &str) -> String {
    let mut result = String::new();

    for (i, char) in title.chars().enumerate() {
        if char == '_' {
            result.push(' ');
            continue;
        }

        if i == 0 {
            result.push(char.to_ascii_uppercase());
            continue;
        }

        result.push(char);
    }

    return result;
}
