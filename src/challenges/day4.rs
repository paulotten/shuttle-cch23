use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SimpleReindeer {
    #[allow(unused)]
    name: String,
    strength: i32,
}

#[derive(Deserialize)]
pub struct ComplexReindeer {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
pub struct ReindeerSummary {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn strength(Json(list): Json<Vec<SimpleReindeer>>) -> String {
    list.iter().map(|r| r.strength).sum::<i32>().to_string()
}

pub async fn contest(Json(list): Json<Vec<ComplexReindeer>>) -> Json<ReindeerSummary> {
    assert!(!list.is_empty());

    let mut fastest = (0, list[0].speed);
    let mut tallest = (0, list[0].height);
    let mut magician = (0, list[0].snow_magic_power);
    let mut consumer = (0, list[0].candies_eaten_yesterday);

    for (i, r) in list.iter().enumerate().skip(1) {
        if r.speed > fastest.1 {
            fastest = (i, r.speed);
        }
        if r.height > tallest.1 {
            tallest = (i, r.height);
        }
        if r.snow_magic_power > magician.1 {
            magician = (i, r.snow_magic_power);
        }
        if r.candies_eaten_yesterday > consumer.1 {
            consumer = (i, r.candies_eaten_yesterday);
        }
    }

    Json(ReindeerSummary {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            list[fastest.0].strength, list[fastest.0].name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            list[tallest.0].name, list[tallest.0].antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            list[magician.0].name, list[magician.0].snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            list[consumer.0].name, list[consumer.0].favorite_food
        ),
    })
}
