use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use roxmltree;

static FLOOR: &'static [&str] = &["intFloor", "flrSPCategory"];

pub struct Item {
    pub name: String,
    pub category: String,
    pub gold: i32,
    pub silver: i32,
    pub rating: i32,
}

pub fn parse_all_clothes() -> HashMap<String, HashMap<String, Item>> {
    let mut out = HashMap::new();
    out.insert("boy".to_owned(), parse_clothes("boy"));
    out.insert("girl".to_owned(), parse_clothes("girl"));
    return out;
}

pub fn parse_clothes(gender: &str) -> HashMap<String, Item> {
    let mut file = File::open(format!("config/inventory/{}Clothes.xml", &gender)).expect("Can't open cloth file");
    let mut xml = String::new();
    file.read_to_string(&mut xml).expect("Can't read cloth file");
    let mut out = HashMap::new();
    let doc = roxmltree::Document::parse(&xml).expect("Can't parse cloth xml");
    for elem in doc.root_element().children() {
        parse_category(elem, &mut out);
    }
    return out;
}

fn parse_category(category: roxmltree::Node, mut out: &mut HashMap<String, Item>) {
    for elem in category.children() {
        match elem.tag_name().name() {
            "category" => parse_category(elem, &mut out),
            "item" => {
                let name = elem.attribute("id").unwrap().to_string();
                let mut cat_name = category.attribute("id").unwrap().to_string();
                if FLOOR.contains(&cat_name.as_str()) {
                    cat_name = "4".to_string();
                }
                let gold = elem.attribute("gold").unwrap_or("0").parse::<i32>().unwrap();
                let silver = elem.attribute("silver").unwrap_or("0").parse::<i32>().unwrap();
                let rating = elem.attribute("rating").unwrap_or("0").parse::<i32>().unwrap();
                let item = Item {
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

pub fn parse_furniture() -> HashMap<String, Item> {
    let mut out = HashMap::new();
    for filename in &["furniture", "kitchen", "bathroom", "decor", "roomLayout"] {
        let mut file = File::open(format!("config/inventory/{}.xml", filename)).expect("Can't open furniture file");
        let mut xml = String::new();
        file.read_to_string(&mut xml).expect("Can't read furniture file");
        let doc = roxmltree::Document::parse(&xml).expect("Can't parse furniture xml");
        for elem in doc.root_element().children() {
            parse_category(elem, &mut out);
        }
    }
    return out;
}

pub fn parse_trophies() -> Vec<String> {
    let mut file = File::open("config/modules/trophies.xml").expect("Can't open trophies file");
    let mut xml = String::new();
    file.read_to_string(&mut xml).expect("Can't read trophies file");
    let mut out = Vec::new();
    let doc = roxmltree::Document::parse(&xml).expect("Can't parse trophies xml");
    for elem in doc.root_element().children() {
        if !elem.is_element() {
            continue;
        }
        println!("{}", elem.tag_name().name());
        out.push(elem.attribute("id").unwrap().to_string());
    }
    return out;
}
