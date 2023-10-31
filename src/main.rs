use rpress::{
    config::{self, HProperty},
    parse,
};
use tera::Tera;
use toml;
fn main() {
    let config_content = config::Config::get_config_content(format!("{}/config.toml", "./"));
    let path_list = parse::get_path_list(&config_content.source);
    std::fs::create_dir_all(format!("{}/Post", &config_content.dest)).expect("无法创建dist");
    for p in path_list.iter() {
        if p.ends_with("markdown") || p.ends_with("md") {
            let md_config_content: HProperty =
                toml::from_str(parse::read_markdown(p).0.as_str()).unwrap();
            let mut context = tera::Context::new();
            context.insert("body", parse::read_markdown(p).1.as_str());
            println!("777{}", parse::read_markdown(p).1.as_str());
            context.insert("post_index", "6");
            context.insert("tags", "6");
            context.insert("categories", "6");
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
            println!("{:?}", &context);
            let rendered = tera
                .render(format!("{}.html", "index").as_str(), &context)
                .unwrap();
            println!("{}", rendered);
            std::fs::write(cur_path, rendered).unwrap();
        }
    }
    println!("Hello, world!{:?}666{:?}", config_content, path_list);
}
