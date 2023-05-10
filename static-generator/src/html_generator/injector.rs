use crate::model::post::Post;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn inject(posts: Vec<Post>) {
    let mut handlebars = Handlebars::new();

    for post in posts {
        let title = post.title.clone();
        handlebars
            .register_template_file(&title, "src/html_generator/template.hbs")
            .unwrap();

        let mut data = BTreeMap::new();
        data.insert("title".to_string(), post.title);
        data.insert("content".to_string(), post.content);
        let result = handlebars.render(&title, &data).unwrap();

        println!("injecting posts: {:?}", result);
        let filepath = "html/".to_string() + &title + ".html";
        let mut file = File::create(filepath).unwrap();
        file.write_all(result.as_bytes()).unwrap()
    }
}
