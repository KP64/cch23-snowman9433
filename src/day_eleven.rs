use image::{GenericImageView, Pixel, Rgb};
use rocket::{
    form::Form, fs::{relative, NamedFile, TempFile}, routes, Route
};
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

pub fn routes() -> Vec<Route> {
    routes![part1, part2]
}

#[rocket::get("/assets/<path..>")]
async fn part1(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("assets")).join(path);

    NamedFile::open(path).await.ok()
}

#[rocket::post("/red_pixels", data = "<file>")]
async fn part2(file: Form<TempFile<'_>>) -> Option<String> {
    let mut buf = vec![];
    let mut opened = file.open().await.ok()?;
    opened.read_to_end(&mut buf).await.ok()?;

    let img = image::load_from_memory(&buf).ok()?;

    img.pixels()
        .filter(|(_, _, color)| {
            let Rgb([r, g, b]) = color.to_rgb();

            r > b.saturating_add(g)
        })
        .count()
        .to_string()
        .into()
}
