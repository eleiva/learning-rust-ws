use serde::Serialize;

#[derive(Serialize)]
pub struct MenuItem {
    title: String,
    key: String,
    order: i8
}

#[derive(Serialize)]
pub struct Menu {
    display: String,
    menu_items: Vec<MenuItem>
}

#[derive(Serialize)]
pub struct Item {
    title: String,
    description: String,
    images: Vec<String>,
    price: f32
}

#[derive(Serialize)]
pub struct Search {
    msg: String,
    display: String,
    items: Vec<Item>,
}

pub fn menu() -> Menu {
    Menu {
        display: "menu".to_string(),
        menu_items: vec![
            MenuItem {
                title: "Buscar items".to_string(),
                key: "buscar <producto>".to_string(),
                order: 1
            },
            MenuItem {
                title: "Menu".to_string(),
                key: "menu".to_string(),
                order: 2
            }
        ]
    }
}

pub fn search(product: String) -> Search {
    Search {
        display: "item_list".to_string(),
        msg: "Primeros 10 articulos encontrados".to_string(),
        items: vec![
            Item {
                title: product.clone(),
                description: "B".to_string(),
                images: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                price: 2.3
            },
            Item {
                title: product.clone(),
                description: "Y".to_string(),
                images: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                price: 4.0
            }
        ]
    }
}