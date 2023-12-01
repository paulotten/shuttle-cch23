use axum::extract::Path;

pub async fn cube_bits(Path(ids): Path<String>) -> String {
    println!("cube_bits: {:?}", ids);

    let ids = parse_ids(ids);
    let mut sum = 0;

    for id in ids {
        sum = xor(sum, id);
    }

    let result = cube(sum);

    format!("{result}")
}

fn parse_ids(s: String) -> Vec<i64> {
    let mut ids = vec![];

    for part in s.split('/') {
        if let Ok(id) = part.parse::<i64>() {
            ids.push(id);
        }
    }

    ids
}

fn xor(a: i64, b: i64) -> i64 {
    a ^ b
}

fn cube(a: i64) -> i64 {
    a.pow(3)
}
