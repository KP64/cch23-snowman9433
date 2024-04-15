use rocket::http::Status;

#[rocket::get("/")]
pub fn part1() -> Status {
    Status::Ok
}

#[rocket::get("/-1/error")]
pub fn part2() -> Status {
    Status::InternalServerError
}
