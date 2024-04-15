use rocket::serde::json::Json;
use serde_json::{json, Value};

#[rocket::post("/", data = "<txt>")]
pub fn solution(mut txt: String) -> Json<Value> {
    let elfs = txt.matches("elf").count();

    let mut elf_on_shelf = 0;
    while txt.contains("elf on a shelf") {
        txt = txt.replacen("elf on a shelf", "elf", 1);
        elf_on_shelf += 1;
    }
    let shelfs_with_no_elfs = txt.matches("shelf").count();

    Json(json!({
        "elf": elfs,
        "elf on a shelf": elf_on_shelf,
        "shelf with no elf on it": shelfs_with_no_elfs
    }))
}
