use std::{thread::sleep, time::Duration};

use hotwatch::{notify::Event, EventKind, Hotwatch};
use rpress::{config, parse::parser};

fn main() {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch("./", |event: Event| {
            if let EventKind::Modify(_) = event.kind {
                let config_content =
                    config::Config::get_config_content(format!("{}/config.toml", "./"));
                parser(config_content.clone());
            }
        })
        .expect("failed to watch file!");

    loop {
        sleep(Duration::from_secs(2));
    }
}
