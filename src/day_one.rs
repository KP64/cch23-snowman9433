use std::path::PathBuf;

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

#[rocket::get("/<num1>/<num2>", rank = 1)]
fn part1(num1: isize, num2: isize) -> String {
    (num1 ^ num2).pow(3).to_string()
}

#[rocket::get("/<nums..>", rank = 2)]
fn part2(nums: PathBuf) -> Option<String> {
    let s = nums.to_str()?;

    #[cfg(target_family = "windows")]
    let it = s.split('\\');

    #[cfg(target_family = "unix")]
    let it = s.split('/');

    it.flat_map(str::parse::<isize>)
        .take(20)
        .reduce(|acc, x| acc ^ x)?
        .pow(3)
        .to_string()
        .into()
}
