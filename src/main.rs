use rocket::routes;

mod day_five;
mod day_four;
mod day_minus_one;
mod day_one;
mod day_seven;
mod day_six;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![day_minus_one::part1, day_minus_one::part2])
        .mount("/1", routes![day_one::part1, day_one::part2])
        .mount("/4", routes![day_four::part1, day_four::part2])
        .mount("/5", routes![day_five::solution])
        .mount("/6", routes![day_six::solution])
        .mount("/7", routes![day_seven::part1, day_seven::part2]);

    Ok(rocket.into())
}
