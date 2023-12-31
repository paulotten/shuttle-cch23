use axum::{
    routing::{get, post},
    Router,
};

mod challenges;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        // minus 1
        .route("/", get(challenges::minus1::http_200))
        .route("/-1/error", get(challenges::minus1::http_500))
        // day 1
        .route("/1/*ids", get(challenges::day1::cube_bits))
        // day 4
        .route("/4/strength", post(challenges::day4::strength))
        .route("/4/contest", post(challenges::day4::contest))
        // day 6
        .route("/6", post(challenges::day6::elf))
        // day 7
        .route("/7/decode", get(challenges::day7::decode))
        .route("/7/bake", get(challenges::day7::bake))
        // day 8
        .route("/8/weight/:id", get(challenges::day8::weight))
        .route("/8/drop/:id", get(challenges::day8::drop));

    Ok(router.into())
}
