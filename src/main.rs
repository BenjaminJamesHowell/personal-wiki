mod args;
mod new;
mod page;
mod server;
mod template;

use crate::server::{category, index, not_found, pages};
use rocket::fs::{relative, FileServer};

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

        _ => {
            panic!("Command not implemented");
        }
    };

    return Ok(());
}
