mod args;
mod build;
mod html;
mod new;
mod page;
mod server;
mod template;

use crate::server::{category, err_404, pages, search};
use rocket::fs::{relative, FileServer};
use server::{redir_to_home_from_pages, redir_to_home_from_root};
use std::fs::read_to_string;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args = args::get_args();

    match args.command {
        args::Command::New(_) => {
            new::new(args);
        }

        args::Command::Build => {
            build::build(args);
        }

        args::Command::Serve => {
            rocket::build()
                .mount(
                    "/",
                    routes![
                        pages,
                        category,
                        search,
                        redir_to_home_from_root,
                        redir_to_home_from_pages
                    ],
                )
                .mount("/assets", FileServer::from("./assets"))
                .mount(
                    "/static-assets",
                    FileServer::from(relative!("./static/assets/")),
                )
                .register("/", catchers![err_404])
                .launch()
                .await?;
            return Ok(());
        }

        args::Command::Help => {
            let info = read_to_string(relative!("./static/help.txt"))
                .expect("Cannot open help information.");
            println!("{}", info);
        }

        args::Command::Version => {
            let info = read_to_string(relative!("./static/version.txt"))
                .expect("Cannot open version information.");
            println!("{}", info);
        }
    };

    return Ok(());
}
