use rocket::{http::Status, routes, Route};

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

#[rocket::get("/")]
const fn part1() -> Status {
    Status::Ok
}

#[rocket::get("/-1/error")]
const fn part2() -> Status {
    Status::InternalServerError
}
