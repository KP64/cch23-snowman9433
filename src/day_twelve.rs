use std::ops::Deref;

use chrono::{DateTime, Datelike, Utc};
use rocket::{routes, serde::json::Json, Route, State};
use serde_json::{json, Value};
use shuttle_persist::PersistInstance;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug)]
pub struct Persistence(pub PersistInstance);

impl Deref for Persistence {
    type Target = PersistInstance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn routes() -> Vec<Route> {
    routes![get_task1, post_task1, task2, task3]
}

#[rocket::get("/load/<string>")]
fn get_task1(string: String, persistance: &State<Persistence>) -> Option<String> {
    let old_time = persistance.load::<DateTime<Utc>>(&string).ok()?;

    let diff = Utc::now() - old_time;
    Some(diff.num_seconds().to_string())
}

#[rocket::post("/save/<string>")]
fn post_task1(string: String, persistance: &State<Persistence>) -> Option<()> {
    persistance.save(&string, Utc::now()).ok()
}

#[rocket::post("/ulids", data = "<arr>")]
fn task2(arr: Json<Vec<String>>) -> Json<Vec<Uuid>> {
    arr.iter()
        .rev()
        .filter_map(|line| {
            let ulid = Ulid::from_string(line).ok()?;
            Some(Uuid::from(ulid))
        })
        .collect::<Vec<_>>()
        .into()
}

#[rocket::post("/ulids/<weekday>", data = "<arr>")]
fn task3(weekday: u32, arr: Json<Vec<String>>) -> Option<Json<Value>> {
    let mut wk_count = 0;
    let mut christmas_eve = 0;
    let mut future = 0;
    let mut lsb = 0;

    let now = Utc::now();
    for line in arr.iter() {
        let ulid = Ulid::from_string(line).ok()?;
        let datetime: DateTime<Utc> = ulid.datetime().into();

        if datetime.month() == 12 && datetime.day() == 24 {
            christmas_eve += 1;
        }

        if datetime.weekday().num_days_from_monday() == weekday {
            wk_count += 1;
        }

        if datetime > now {
            future += 1;
        }

        if ulid.to_bytes().last()? & 1 == 1 {
            lsb += 1;
        }
    }

    Some(Json(json!({
        "christmas eve": christmas_eve,
        "weekday": wk_count,
        "in the future": future,
        "LSB is 1": lsb
    })))
}
