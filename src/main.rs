mod args;
mod new;
mod page;
mod server;
mod template;

use crate::server::{category, index, not_found, pages};
use rocket::fs::{relative, FileServer};
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

        args::Command::Serve => {
            rocket::build()
                .mount("/api", routes![index, pages, category])
                .mount("/public", FileServer::from(relative!("./static")))
                .mount("/assets", FileServer::from("./assets"))
                .register("/public", catchers![not_found])
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
