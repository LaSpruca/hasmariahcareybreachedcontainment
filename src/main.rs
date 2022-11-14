use chrono::{Datelike, Utc};
use std::{net::SocketAddr, str::FromStr};
use warp::Filter;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let template =
        include_str!("../index.html").replace("%site%", "hasmariahcareybreachedcontainment.com");

    let robots = warp::path("robots.txt")
        .map(|| "User-Agent: *\nAllow: /")
        .with(warp::reply::with::header("Content-Type", "text/plain"))
        .with(warp::log("Robots"));

    let site = warp::any()
        .map(move || {
            let now = Utc::now();

            if now.month() == 11 || now.month() == 12 {
                template
                    .replace("%breached%", "Yes")
                    .replace("%color%", "#eee")
                    .replace("%bg%", "#222")
            } else {
                template
                    .replace("%breached%", "No")
                    .replace("%color%", "#222")
                    .replace("%bg%", "#fff")
            }
        })
        .with(warp::reply::with::header(
            "Content-Type",
            "text/html; charset=UTF-8",
        ))
        .with(warp::log("Site"));

    warp::serve(robots.or(site))
        .bind(SocketAddr::from_str("0.0.0.0:8000").unwrap())
        .await;
}
