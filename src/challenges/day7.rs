use std::collections::HashMap;

use axum::http::HeaderMap;
use base64::{engine::general_purpose, Engine as _};
use serde_json::json;

pub async fn decode(headers: HeaderMap) -> String {
    let cookie_header = headers.get("Cookie").unwrap().to_str().unwrap();
    let encoded_string = cookie_header.replace("recipe=", "");

    let decoded_bytes = general_purpose::STANDARD.decode(encoded_string).unwrap();

    String::from_utf8(decoded_bytes).unwrap()
}

pub async fn bake(headers: HeaderMap) -> String {
    let json_string = decode(headers).await;

    let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    let json = json.as_object().unwrap();
    let recipe_json = json.get("recipe").unwrap();
    let pantry_json = json.get("pantry").unwrap();
    let recipe_json = recipe_json.as_object().unwrap();
    let pantry_json = pantry_json.as_object().unwrap();

    let mut cookies: u64 = 0;

    // get list of ingredients in recipe and pantry
    let mut recipe: HashMap<&str, u64> = HashMap::new();

    for ingredient in recipe_json.keys() {
        let amount = recipe_json.get(ingredient).unwrap().as_u64().unwrap();

        recipe.insert(ingredient, amount);
    }

    let mut pantry: HashMap<&str, u64> = HashMap::new();

    for ingredient in pantry_json.keys() {
        let amount = pantry_json.get(ingredient).unwrap().as_u64().unwrap();

        pantry.insert(ingredient, amount);
    }

    // while enough ingredients in pantry, make cookies
    while have_ingredients(&recipe, &pantry) {
        // remove ingrediens from pantry
        for ingredient in recipe.keys() {
            let recipe_amount = recipe.get(ingredient).unwrap();
            let pantry_amount = pantry.get(ingredient).unwrap();

            let pantry_amount = pantry_amount - recipe_amount;
            pantry.insert(ingredient, pantry_amount);
        }

        // make a cookie
        cookies += 1;
    }

    let mut output_map = serde_json::Map::new();
    output_map.insert("cookies".to_string(), json!(cookies));
    output_map.insert("pantry".to_string(), json!(pantry));

    let output_json: serde_json::Value = serde_json::Value::Object(output_map);

    output_json.to_string()
}

fn have_ingredients(recipe: &HashMap<&str, u64>, pantry: &HashMap<&str, u64>) -> bool {
    for ingredient in recipe.keys() {
        if recipe.get(ingredient).unwrap_or(&0) > pantry.get(ingredient).unwrap_or(&0) {
            return false;
        }
    }

    true
}
