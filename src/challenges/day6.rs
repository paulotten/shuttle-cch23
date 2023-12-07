use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

pub async fn elf(body: String) -> Json<ElfCount> {
    let elf = body.matches("elf").count();
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let shelf = body.matches("shelf").count();
    let shelf_with_no_elf = shelf - elf_on_a_shelf;

    Json(ElfCount {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf,
    })
}
