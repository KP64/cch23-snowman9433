use rocket::{http::RawStr, routes, serde::json::Json, Route};
use serde::Deserialize;

pub fn routes() -> Vec<Route> {
    routes![task1, task2]
}

#[derive(Deserialize)]
struct Content {
    content: String,
}

#[rocket::post("/unsafe", data = "<content>")]
fn task1(content: Json<Content>) -> String {
    render(&content.content)
}

#[rocket::post("/safe", data = "<content>")]
fn task2(content: Json<Content>) -> String {
    let raw_str: &RawStr = content.content.as_str().into();
    let escaped = raw_str.html_escape();

    // Rocket does "more than necessary"
    let escaped = escaped.replace("&#x2F;", "/");

    render(&escaped)
}

fn render(content: &str) -> String {
    format!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {content}
  </body>
</html>"
    )
}
