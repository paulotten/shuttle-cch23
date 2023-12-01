use axum::{routing::get, Router};

mod challenges;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        // minus 1
        .route("/", get(challenges::minus1::http_200))
        .route("/-1/error", get(challenges::minus1::http_500))
        // day 1
        .route("/1/*ids", get(challenges::day1::cube_bits));

    Ok(router.into())
}
