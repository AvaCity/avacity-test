use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use roxmltree;
use crate::modules::avatar::Cloth;

pub fn parse_clothes(gender: &str) -> HashMap<String, Cloth> {
    let mut file = File::open(format!("config/inventory/{}Clothes.xml", &gender)).expect("Can't open cloth file");
    let mut xml = String::new();
    file.read_to_string(&mut xml).expect("Can't read cloth file");
    let mut out = HashMap::new();
    let doc = roxmltree::Document::parse(&xml).expect("Can't parse cloth xml");
    for elem in doc.root_element().children() {
        parse_cloth_category(elem, &mut out);
    }
    return out;
}

fn parse_cloth_category(category: roxmltree::Node, mut out: &mut HashMap<String, Cloth>) {
    for elem in category.children() {
        match elem.tag_name().name() {
            "category" => parse_cloth_category(elem, &mut out),
            "item" => {
                let name = elem.attribute("id").unwrap().to_string();
                let cat_name = category.attribute("id").unwrap().to_string();
                let gold = elem.attribute("gold").unwrap_or("0").parse::<i32>().unwrap();
                let silver = elem.attribute("silver").unwrap_or("0").parse::<i32>().unwrap();
                let rating = elem.attribute("rating").unwrap_or("0").parse::<i32>().unwrap();
                let item = Cloth {
                    name: name.clone(),
                    category: cat_name.clone(),
                    gold: gold,
                    silver: silver,
                    rating: rating
                };
                out.insert(name, item);
            },
            _ => continue
        }
    }
}
