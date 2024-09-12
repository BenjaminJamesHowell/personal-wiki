pub fn config_json(name: &str) -> String {
    return format!(
        r#"{{
    "name": "{}",
}}"#,
        name,
    );
}

pub fn example_page_md() -> String {
    return format!(
        r#"
This is an example page.
"#
    );
}

pub fn page_md(name: &str) -> String {
    return format!(
        r#"# {}

Blank page
"#,
        name,
    );
}
