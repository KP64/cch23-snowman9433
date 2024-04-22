use rocket::{http::Status, routes, serde::json::Json, Route, State};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, Executor, PgPool};
use std::ops::Deref;

pub struct DB(pub PgPool);

impl Deref for DB {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn routes() -> Vec<Route> {
    routes![
        task1,
        task2_reset,
        task2_post_orders,
        task2_get_orders_total,
        task3_get_popular_orders
    ]
}

#[rocket::get("/sql")]
async fn task1(db: &State<DB>) -> Option<String> {
    #[derive(FromRow)]
    struct Res(i32);

    let res = sqlx::query_as::<_, Res>("SELECT 20231213")
        .fetch_one(&db.0)
        .await
        .ok()?;

    Some(res.0.to_string())
}

#[rocket::post("/reset")]
async fn task2_reset(db: &State<DB>) -> Status {
    if db.execute(include_str!("../schema.sql")).await.is_ok() {
        Status::Ok
    } else {
        Status::InternalServerError
    }
}

#[derive(Deserialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[rocket::post("/orders", data = "<orders>")]
async fn task2_post_orders(orders: Json<Vec<Order>>, db: &State<DB>) -> Option<()> {
    for order in orders.0 {
        sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name)
        .bind(order.quantity)
        .execute(&db.0)
        .await
        .ok()?;
    }
    Some(())
}

#[rocket::get("/orders/total")]
async fn task2_get_orders_total(db: &State<DB>) -> Option<Json<Value>> {
    #[derive(FromRow)]
    struct Res(i64);

    let sum = sqlx::query_as::<_, Res>(
        "SELECT SUM(quantity)
          FROM orders",
    )
    .fetch_one(&db.0)
    .await
    .ok()?;

    Some(Json(json!({
        "total": sum.0
    })))
}

#[rocket::get("/orders/popular")]
async fn task3_get_popular_orders(db: &State<DB>) -> Json<Value> {
    let popular_toy = sqlx::query_as::<_, (String, i64)>(
        "SELECT tmp.gift_name, MAX(tmp.c)
        FROM (
            SELECT gift_name, COUNT(id) AS c
            FROM orders
            GROUP BY gift_name
        ) AS tmp
        GROUP BY tmp.gift_name",
    )
    .fetch_optional(&db.0)
    .await
    .unwrap();

    let Some(name) = popular_toy else {
        return Json(json!({
            "popular": null
        }));
    };

    Json(json!({
        "popular": name.0
    }))
}
