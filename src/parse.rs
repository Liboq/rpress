use std::collections::HashMap;

// markdown 转 html
use markdown::{to_html_with_options, to_mdast, CompileOptions, Options, ParseOptions};
use tera::Tera;

use crate::config::{Config, HProperty, HpropertyString};

pub fn parse_md_2_html(md: &str) -> (String, HashMap<String, usize>) {
    let html_body = to_html_with_options(
        md,
        &Options {
            parse: ParseOptions::gfm(),
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
            ..Options::default()
        },
    )
    .expect("err markdown");
    let ast = to_mdast(md, &ParseOptions::default()).unwrap();
    // println!("{:?}", ast);
    // let block_arr = tokenize(md);
    let anchor_list = HashMap::new();
    // for block in block_arr {
    //     match block {
    //         Block::Header(_text, _index) => match _text[0].clone() {
    //             Span::Text(text) => {
    //                 anchor_list.insert(text.clone(), _index);
    //             }
    //             _ => (),
    //         },
    //         _ => (),
    //     }
    // }
    (html_body, anchor_list)
}
/* md 2 html
 * 我们只生成toml, body
*/
pub fn read_markdown(md_file: &str) -> (String, String, HashMap<String, usize>) {
    //println!("{}", md_file);
    let raw_text = std::fs::read_to_string(md_file).expect(md_file);
    let cut_raw: Vec<&str> = raw_text.split("---").collect();
    let toml_text = cut_raw[1];
    let toml_t = toml_text;
    /* TOML is OK */
    let md_raw = &cut_raw[2..];
    let mut md_text: String = "".to_string();
    for md in md_raw {
        md_text.push_str(&md);
    }

    // let parser = pulldown_cmark::Parser::new_ext(&md_text, pulldown_cmark::Options::all());
    // let mut body = String::new();
    // pulldown_cmark::html::push_html(&mut body, parser);
    /*BODY is OK */
    let (body, anchorlist) = parse_md_2_html(&md_text);
    (toml_t.to_string(), body, anchorlist)
}
/*
获取markdown文件列表
 */
pub fn get_path_list(path: &str) -> Vec<String> {
    let mut my_filename_list: Vec<String> = vec![];
    // 只需要文件及对应的路径，不需要空文件夹的名称及路径
    for e in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            //println!("{}", e.path().display());
            my_filename_list.push(e.path().display().to_string());
        } else {
            // crate::utils::info(
            //     crate::utils::Info::GENERATE,
            //     "found dectory",
            //     e.path().display().to_string().as_str(),
            // );
        }
    }
    my_filename_list
}
pub fn parser(config_content: Config) {
    let path_list = get_path_list(&config_content.source);
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
            let mut context = tera::Context::new();
            let md_all = read_markdown(p);
            let md_config_content: HProperty = toml::from_str(md_all.0.as_str()).unwrap();
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

