use regex::Regex;
use rocket::{http::Status, serde::json::Json, Route};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub fn routes() -> Vec<Route> {
    rocket::routes![task1, task2]
}

#[derive(Deserialize)]
struct Input {
    input: String,
}

#[rocket::post("/nice", data = "<input>")]
fn task1(input: Json<Input>) -> (Status, Json<Value>) {
    const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    const FORBIDDEN: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];

    let passwd = &input.input;

    let vow_count = passwd.chars().filter(|c| VOWELS.contains(c)).count();
    let has_forbidden = passwd.as_bytes().windows(2).any(|w| FORBIDDEN.contains(&w));
    let has_adjacent = passwd
        .chars()
        .zip(passwd.chars().skip(1))
        .any(|(a, b)| a.is_alphabetic() && a == b);

    if vow_count >= 3 && !has_forbidden && has_adjacent {
        (
            Status::Ok,
            Json(json!({
                "result": "nice"
            })),
        )
    } else {
        (
            Status::BadRequest,
            Json(json!({
                "result": "naughty"
            })),
        )
    }
}

#[rocket::post("/game", data = "<input>")]
fn task2(input: Json<Input>) -> (Status, Json<Value>) {
    let passwd = &input.input;

    let uppercase = Regex::new(r"[A-Z]+").unwrap();
    let lowercase = Regex::new(r"[a-z]+").unwrap();
    let digit = Regex::new(r"[0-9]").unwrap();
    let number = Regex::new(r"[0-9]+").unwrap();
    let emoji = Regex::new(concat!(
        "[",
        "\u{01F600}-\u{01F64F}",
        "\u{01F300}-\u{01F5FF}",
        "\u{01F680}-\u{01F6FF}",
        "\u{01F1E0}-\u{01F1FF}",
        "\u{002702}-\u{0027B0}",
        "]+",
    ))
    .unwrap();

    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, passwd);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    if passwd.len() < 8 {
        return bad_req_with_reason(Status::BadRequest, "8 chars");
    }

    if !(uppercase.is_match(passwd) && lowercase.is_match(passwd) && digit.is_match(passwd)) {
        return bad_req_with_reason(Status::BadRequest, "more types of chars");
    }

    if digit.find_iter(passwd).count() < 5 {
        return bad_req_with_reason(Status::BadRequest, "55555");
    }

    if number
        .find_iter(passwd)
        .flat_map(|m| m.as_str().parse::<usize>())
        .sum::<usize>()
        != 2023
    {
        return bad_req_with_reason(Status::BadRequest, "math is hard");
    }

    if !contains_joy(passwd) {
        return bad_req_with_reason(Status::NotAcceptable, "not joyful enough");
    }

    if !passwd
        .chars()
        .zip(passwd.chars().skip(2))
        .any(|(a, b)| a.is_alphabetic() && a == b)
    {
        return bad_req_with_reason(Status::UnavailableForLegalReasons, "illegal: no sandwich");
    }

    if !passwd.chars().any(|c| matches!(c, '\u{2980}'..='\u{2BFF}')) {
        return bad_req_with_reason(Status::RangeNotSatisfiable, "outranged");
    }

    if !emoji.is_match(passwd) {
        return bad_req_with_reason(Status::UpgradeRequired, "😳");
    }

    if !hash_hex.ends_with('a') {
        return bad_req_with_reason(Status::ImATeapot, "not a coffee brewer");
    }

    (
        Status::Ok,
        Json(json!({
            "result": "nice",
            "reason": "that's a nice password",
        })),
    )
}

fn contains_joy(passwd: &str) -> bool {
    let j = passwd.match_indices('j');
    let o = passwd.match_indices('o');
    let y = passwd.match_indices('y');
    if j.clone().count() != 1 || o.clone().count() != 1 || y.clone().count() != 1 {
        false
    } else {
        j.min() < o.clone().min() && o.min() < y.min()
    }
}

fn bad_req_with_reason(status: Status, reason: &str) -> (Status, Json<Value>) {
    (
        status,
        Json(json!({
            "result": "naughty",
            "reason": reason
        })),
    )
}
