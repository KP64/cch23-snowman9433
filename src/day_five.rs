use rocket::serde::json::Json;

#[rocket::post("/?<offset>&<limit>&<split>", data = "<arr>")]
pub fn solution(
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
    arr: Json<Vec<String>>,
) -> Option<String> {
    let offset = offset.unwrap_or_default();
    let limit = limit.unwrap_or_else(|| arr.len() - offset);

    let sliced = arr
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect::<Vec<_>>();

    let Some(split) = split else {
        return serde_json::to_string(&sliced).ok();
    };

    let splitted = sliced.chunks(split).collect::<Vec<_>>();
    serde_json::to_string(&splitted).ok()
}
