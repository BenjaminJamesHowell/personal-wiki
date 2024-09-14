use crate::args::{Args, Command, NewThing};
use inquire::error::InquireResult;
use std::{
    env,
    fs::{create_dir, write},
};

pub fn new(args: Args) {
    match args.command {
        Command::New(NewThing::Wiki) => {
            let questions = ask_questions().unwrap();

            create_files(questions).unwrap();
        }

        Command::New(NewThing::Page(name)) => {
            create_page(&name).unwrap();
        }

        _ => {
            panic!("");
        }
    }
}

#[derive(Debug)]
struct Questions {
    pub name: Option<String>,
    pub is_new_dir: bool,
}

fn ask_questions() -> InquireResult<Questions> {
    let is_new_dir_options = vec![
        "Create the wiki in this directory",
        "Create the wiki in a new sub-directory",
    ];
    let is_new_dir = inquire::Select::new("", is_new_dir_options).prompt()?;
    let is_new_dir = match is_new_dir {
        "Create the wiki in this directory" => false,
        "Create the wiki in a new sub-directory" => true,
        _ => panic!("Invalid answer"),
    };

    let name = match is_new_dir {
        true => Some(inquire::Text::new("Enter wiki name: ").prompt()?),
        false => None,
    };

    return Ok(Questions { is_new_dir, name });
}

fn create_files(questions: Questions) -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let parent_dir = cwd.file_name().unwrap().to_str().unwrap().to_owned();

    // While "name" is always the name of the wiki's parent directory, "dir" is the location of
    // that parent directory relative to the cwd.
    // "dir" can equal "." if the wiki is created in the cwd.
    let name = questions.name.clone().unwrap_or(parent_dir);
    let dir = questions.name.unwrap_or(".".to_owned());

    if questions.is_new_dir {
        create_dir(&dir)?;
    }

    write(
        dir.clone() + "/config.json",
        crate::template::config_json(&name),
    )?;
    create_dir(dir.clone() + "/pages")?;
    create_dir(dir.clone() + "/assets")?;
    write(
        dir.clone() + "/pages/example_page",
        crate::template::example_page_md(),
    )?;

    return Ok(());
}

fn create_page(name: &str) -> std::io::Result<()> {
    write(
        String::from("./pages/") + name,
        crate::template::page_md(name),
    )?;

    return Ok(());
}
