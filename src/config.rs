use serde::{Deserialize, Serialize};
use std::fs;
use toml::{self, value::Datetime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavbarItem {
    pub text: String,
    pub link: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Navbar {
    pub text: String,
    pub link: String,
    pub children: Vec<NavbarItem>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub base: String,
    pub title: String,
    pub description: String,
    // head:Vec<>
    pub host: String,
    pub port: i32,
    pub dest: String,
    pub navbar: Vec<Navbar>,
    pub source:String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HProperty {
    pub title: String,
    pub datetime: Datetime,
    pub tags: Vec<String>,
    pub category: String,
    pub url_name: String,
}
impl Config {
    pub fn get_config_content(path: String) -> Config {
        let config = fs::read_to_string(path).unwrap();
        toml::from_str(config.as_str()).expect("config can't read")
    }
}
