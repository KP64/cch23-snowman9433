use base64::prelude::*;
use rocket::{http::CookieJar, routes, Route};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

#[rocket::get("/decode")]
fn part1(cookies: &CookieJar) -> Option<String> {
    let cookie = cookies.get("recipe")?;
    let val = cookie.value();

    let cook = BASE64_URL_SAFE.decode(val).ok()?;
    let cook = String::from_utf8(cook).ok()?;

    Some(cook)
}

#[derive(Deserialize, Serialize)]
struct Resp {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[rocket::get("/bake")]
fn part2(cookies: &CookieJar) -> Option<Value> {
    let cookie = cookies.get("recipe")?;
    let val = cookie.value();

    let cook = BASE64_URL_SAFE.decode(val).ok()?;
    let cook = String::from_utf8(cook).ok()?;

    let mut response = serde_json::from_str::<Resp>(&cook).unwrap();

    let mut cookie_counter = 0;
    response.recipe.retain(|_, amount| *amount > 0);

    while response.has_enough_ingredients() {
        for (name, amount) in &response.recipe {
            let ingredient = response.pantry.get_mut(name).unwrap();
            if *ingredient < *amount {
                break;
            }
            *ingredient -= amount;
        }

        cookie_counter += 1;
    }

    Some(json!({
        "cookies": cookie_counter,
        "pantry": response.pantry
    }))
}

impl Resp {
    fn has_enough_ingredients(&mut self) -> bool {
        self.recipe
            .iter()
            .filter(|&(name, amount)| {
                let Some(n) = self.pantry.get(name) else {
                    return false;
                };
                n >= amount
            })
            .count()
            == self.recipe.len()
    }
}
