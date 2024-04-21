use rocket::{routes, Route};
use rustemon::{client::RustemonClient, model::pokemon::Pokemon, pokemon::pokemon};
use std::{
    fmt,
    ops::{Div, Mul},
};

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

async fn get_pokemon(pokedex_number: i64) -> Result<Pokemon, rustemon::error::Error> {
    pokemon::get_by_id(pokedex_number, &RustemonClient::default()).await
}

#[rocket::get("/weight/<pokedex_number>")]
async fn part1(pokedex_number: i64) -> Option<String> {
    let pokemon = get_pokemon(pokedex_number).await.ok()?;

    let weight = Hectos(pokemon.weight as f64);
    let weight: Kilos = weight.into();
    Some(weight.to_string())
}

#[rocket::get("/drop/<pokedex_number>")]
async fn part2(pokedex_number: i64) -> Option<String> {
    const GRAVITY: f64 = 9.825;
    const METERS: f64 = 10.0;
    let pokemon = get_pokemon(pokedex_number).await.ok()?;
    let weight = Hectos(pokemon.weight as f64);

    let speed = (2.0 * GRAVITY * METERS).sqrt();
    let weight: Kilos = weight.into();

    let momentum = weight * speed;
    Some(momentum.to_string())
}

struct Kilos(f64);
struct Hectos(f64);

impl Mul<f64> for Kilos {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl fmt::Display for Kilos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Hectos> for Kilos {
    fn from(value: Hectos) -> Self {
        Self(value / 10.0)
    }
}

impl Div<f64> for Hectos {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        self.0 / rhs
    }
}
