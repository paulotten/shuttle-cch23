use axum::extract::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimplePokemon {
    #[allow(unused)]
    name: String,
    weight: u32,
}

pub async fn weight(Path(id): Path<u32>) -> String {
    let weight = get_weight(id).await;

    format!("{weight}")
}

pub async fn drop(Path(id): Path<u32>) -> String {
    let weight = get_weight(id).await;
    let gravity = 9.825;
    let height = 10.0;
    #[allow(clippy::unnecessary_cast)] // compiler needs the hint
    let time = (2.0 * height / gravity as f64).sqrt();
    let speed = gravity * time;
    let momentum = weight * speed;

    format!("{momentum}")
}

async fn get_weight(id: u32) -> f64 {
    let body = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}/"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let pokemon: SimplePokemon = serde_json::from_str(&body).unwrap();

    // hectograms to kg
    pokemon.weight as f64 / 10.0
}
