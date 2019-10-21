#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate config;
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;
extern crate sloggers;

mod logger;
mod server;
mod settings;

use logger::Logger;
use server::Server;
use settings::Settings;

fn main() {
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("Error trying to setup configuration. Error: {}", err),
    };

    let logger = match Logger::new(&settings.logger.level) {
        Ok(value) => value,
        Err(err) => panic!("Error trying to setup logger. Error: {}", err),
    };

    let server = match Server::new(&settings) {
        Ok(value) => value,
        Err(err) => panic!("Error trying to setup server. Error: {}", err),
    };

    logger.info(&format!("Starting sever at {}", &settings.server));

    server.launch();
}
