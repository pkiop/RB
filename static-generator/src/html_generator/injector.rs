use crate::model::post::Post;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn inject(posts: Vec<Post>) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "src/html_generator/template.hbs")
        .unwrap();


    let mut data = BTreeMap::new();
    let title = "My Blog".to_string();
    data.insert("title".to_string(), "My Blog".to_string());
    data.insert("content".to_string(), "Hello, world!".to_string());
    let result = handlebars.render("index", &data).unwrap();

    println!("injecting posts: {:?}", result);
    let mut file = File::create(title + ".html").unwrap();
    file.write_all(result.as_bytes()).unwrap()
}
