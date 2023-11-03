use rpress::{config, parse, server::dev};

#[tokio::main]
async fn main() {
    let config_content = config::Config::get_config_content(format!("{}/config.toml", "./"));
    parse::parser(config_content.clone());
    dev(config_content).await;
}
