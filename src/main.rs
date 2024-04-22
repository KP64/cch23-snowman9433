use shuttle_persist::PersistInstance;
use sqlx::PgPool;

mod day_eight;
mod day_eleven;
mod day_five;
mod day_four;
mod day_minus_one;
mod day_one;
mod day_seven;
mod day_six;
mod day_thirteen;
mod day_twelve;

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    let persistence = day_twelve::Persistence(persist);
    let db = day_thirteen::DB(pool);
    let rocket = rocket::build()
        .mount("/", day_minus_one::routes())
        .mount("/1", day_one::routes())
        .mount("/4", day_four::routes())
        .mount("/5", day_five::routes())
        .mount("/6", day_six::routes())
        .mount("/7", day_seven::routes())
        .mount("/8", day_eight::routes())
        .mount("/11", day_eleven::routes())
        .mount("/12", day_twelve::routes())
        .mount("/13", day_thirteen::routes())
        .manage(persistence)
        .manage(db);

    Ok(rocket.into())
}
