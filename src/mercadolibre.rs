use serde::{Deserialize, Serialize};
extern crate reqwest;

#[derive(Serialize)]
pub struct MenuItem {
    title: String,
    key: String,
    order: i8,
}

#[derive(Serialize)]
pub struct Menu {
    display: String,
    menu_items: Vec<MenuItem>,
}

#[derive(Serialize)]
pub struct Item {
    title: String,
    link: String,
    images: Vec<String>,
    price: f32,
}

#[derive(Serialize)]
pub struct Search {
    msg: String,
    display: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MLItem {
    id: String,
    title: String,
    permalink: String,
    thumbnail: String,
    price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MLSearchResult {
    results: Vec<MLItem>,
}

pub fn menu() -> Menu {
    Menu {
        display: "menu".to_string(),
        menu_items: vec![MenuItem {
            title: "Buscar items".to_string(),
            key: "buscar gaseaosa 7up ".to_string(),
            order: 1,
        }],
    }
}

fn search_in_ml(q: String) -> MLSearchResult {
    let url = format!("https://api.mercadolibre.com/sites/MLA/search?q={q}&sort=price_asc&limit=5");

    let data: String = reqwest::blocking::get(url).unwrap().text().unwrap();

    serde_json::from_str(&data.clone()).unwrap()
}

pub fn search(q: String) -> Search {
    let result: MLSearchResult = search_in_ml(q.clone());

    let items = result
        .results
        .into_iter()
        .map(|item: MLItem| Item {
            title: item.title,
            images: vec![item.thumbnail],
            price: item.price,
            link: item.permalink,
        })
        .collect();

    Search {
        display: "item_list".to_string(),
        msg: format!("Primeros 10 articulos encontrados para: `{q}`"),
        items: items,
    }
}
