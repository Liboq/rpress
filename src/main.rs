use hotwatch::{self, Event, EventKind, Hotwatch};
use rpress::{
    config::{self, Config, HProperty, HpropertyString},
    parse,
    server::dev,
};
use tera::Tera;
use toml;

#[tokio::main]
async fn main() {
    let config_content = config::Config::get_config_content(format!("{}/config.toml", "./"));
    println!("667888812esdadasddsdsada");
    parser(config_content.clone());
    // runner();
    dev(config_content).await;
}
fn parser(config_content: Config) {
    let path_list = parse::get_path_list(&config_content.source);
    let mut tags = Vec::new();
    let mut categories = Vec::new();
    let mut posts_index = Vec::new();
    let mut cur_base = String::new();
    if config_content.enviroment == "dev" {
        cur_base = String::from(".");
    }
    if config_content.enviroment == "prod" {
        cur_base = String::from(&config_content.base);
    }
    std::fs::create_dir_all(format!("{}/Post", &config_content.dest)).expect("无法创建dist");
    for p in path_list.iter() {
        if p.ends_with("markdown") || p.ends_with("md") {
            let md_config_content: HProperty =
                toml::from_str(parse::read_markdown(p).0.as_str()).unwrap();
            let mut context = tera::Context::new();
            let md_all = parse::read_markdown(p);
            let md_config: HProperty = toml::from_str(md_all.0.as_str()).unwrap();
            let md_configs = HpropertyString::new(md_config.clone());
            let md_content = md_all.1.as_str();

            for ite in md_config.tags {
                if !tags.contains(&ite) {
                    tags.push(ite);
                }
            }
            if !categories.contains(&md_config.category) {
                categories.push(md_config.category)
            }
            posts_index.push(md_configs.clone());
            context.insert("props", &md_configs);
            context.insert("body", &md_content);
            context.insert("config", &config_content);
            context.insert("cur_base", &cur_base);

            let mut tera = match Tera::new(format!("{}/{}/*.html", "./", "template").as_str()) {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            let cur_path = format!(
                "./{}/Post/{}.html",
                &config_content.dest, md_config_content.url_name
            );
            tera.autoescape_on(vec!["js"]);
            let rendered = tera
                .render(format!("{}.html", "post").as_str(), &context)
                .unwrap();
            std::fs::write(cur_path, rendered).unwrap();
        }
    }
    let main_template_list = vec!["index", "friend", "tag", "category", "about"];
    for template_name in main_template_list.iter() {
        let mut context = tera::Context::new();
        context.insert("posts_index", &posts_index);
        context.insert("tags", &tags);
        context.insert("categories", &categories);
        context.insert("config", &config_content);
        context.insert("cur_base", &cur_base);
        let mut tera = match Tera::new(format!("{}/{}/*.html", "./", "template").as_str()) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        let cur_path = format!("./{}/{}.html", &config_content.dest, template_name);
        tera.autoescape_on(vec!["js"]);
        let rendered = tera
            .render(format!("{}.html", template_name).as_str(), &context)
            .unwrap();
        std::fs::write(cur_path, rendered).unwrap();
    }
}

fn runner() {
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize");
    hotwatch
        .watch("../../../src", |event: Event| {
            println!("get some event {:?}",event);
            if let EventKind::Modify(_) = event.kind {
                println!("{:?} changed!", event.paths[0]);
            }
        })
        .expect("failed to watch file!");
}
