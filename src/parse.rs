// markdown 转 html
use markdown::{ParseOptions, CompileOptions, Options};

pub fn parse_md_2_html(md: &str) -> String {
    let html_body = markdown::to_html_with_options(
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
    html_body
}
/* md 2 html
 * 我们只生成toml, body
*/
pub fn read_markdown(md_file: &str) -> (String, String) {
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

    let body = parse_md_2_html(&md_text);
    (toml_t.to_string(), body)
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
