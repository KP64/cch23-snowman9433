use rocket::{routes, serde::{json::Json, Deserialize}, Route};
use serde_json::{json, Value};

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

#[derive(Deserialize)]
struct Reindeer<'a> {
    name: &'a str,
    strength: usize,
    speed: Option<f64>,
    height: Option<usize>,
    antler_width: Option<usize>,
    snow_magic_power: Option<usize>,
    favorite_food: Option<&'a str>,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten: Option<usize>,
}

#[rocket::post("/strength", data = "<reindeers>")]
fn part1(reindeers: Json<Vec<Reindeer>>) -> String {
    reindeers
        .iter()
        .map(|r| r.strength)
        .sum::<usize>()
        .to_string()
}

#[rocket::post("/contest", data = "<reindeers>")]
fn part2(reindeers: Json<Vec<Reindeer>>) -> Option<Json<Value>> {
    let fastest = reindeers.iter().max_by(|x, y| {
        x.speed
            .partial_cmp(&y.speed)
            .unwrap_or(std::cmp::Ordering::Less)
    })?;

    let tallest = reindeers.iter().max_by_key(|r| r.height)?;
    let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power)?;
    let consumer = reindeers.iter().max_by_key(|r| r.candies_eaten)?;

    Some(Json(json!({
            "fastest": format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.strength, fastest.name
            ),
            "tallest": format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.name, tallest.antler_width?
            ),
            "magician": format!(
                "{} could blast you away with a snow magic power of {}",
                magician.name, magician.snow_magic_power?
            ),
            "consumer": format!(
                "{} ate lots of candies, but also some {}",
                consumer.name, consumer.favorite_food?
            ),
    })))
}
