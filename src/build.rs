use crate::args::Args;
use crate::html::{self, PageHTML};
use rocket::fs::relative;
use std::collections::HashSet;
use std::fs::{self, copy, create_dir, read_dir, remove_dir_all, write};
use std::path::Path;

pub fn copy_dir_all(target: &Path, new_path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(&new_path)?;

    for file in fs::read_dir(&target).unwrap() {
        let file = file?;

        if file.file_type()?.is_dir() {
            copy_dir_all(&file.path(), &new_path.join(file.file_name()))?;

            continue;
        }

        copy(file.path(), &new_path.join(file.file_name()))?;
    }

    return Ok(());
}

pub fn build(args: Args) {
    remove_dir_all("./build").expect("Cannot remove build directory");
    create_dir("./build").expect("Cannot create build directory");

    create_dir("./build/pages").expect("Cannot create pages directory");
    create_dir("./build/categories").expect("Cannot create categories directory");

    copy_dir_all(
        Path::new(relative!("./static/assets/")),
        Path::new("./build/static-assets"),
    )
    .expect("Cannot copy static assets directory");
    copy_dir_all(Path::new("./assets"), Path::new("./build/assets"))
        .expect("Cannot copy assets directory");

    let mut categories: HashSet<String> = HashSet::new();
    let pages = read_dir("./pages").expect("Cannot read pages directory");

    for page in pages {
        let page = page.expect("Cannot read pages directory");

        let page_title = page.file_name();
        let page_title = page_title.to_str().expect("Cannot read pages directory");

        let PageHTML { output, fm } = html::page(page_title).expect("Cannot build page");
        for category_name in fm.categories {
            categories.insert(category_name);
        }

        let out_path = String::from("./build/pages/") + page_title + ".html";

        write(out_path, output).expect("Cannot write output file");
    }

    for category_name in categories {
        let output = html::category(&category_name).expect("Cannot build category list");
        let out_path = String::from("./build/categories/") + &category_name + ".html";

        write(out_path, output).expect("Cannot write output file");
    }

    let search_output = html::search().expect("Cannot get search results");
    write("./build/search", search_output).expect("Cannot write search results file");
}
