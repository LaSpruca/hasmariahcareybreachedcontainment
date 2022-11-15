use chrono::{Datelike, Utc};
use serde::Serialize;
use worker::*;

mod utils;

const ROBOTS_TXT: &'static str = include_str!("../content/robots.txt");
const IMG_PNG: &'static [u8] = include_bytes!("../content/img.png");

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[derive(Serialize)]
struct TemplateData {
    url: String,
    title: String,
    breached: String,
}

impl TemplateData {
    fn new(breached: String) -> Self {
        Self {
            url: "https://hasmariahcareybreachedcontainment.com".to_string(),
            title: "Has Mariah Carey Breached Containment?".to_string(),
            breached,
        }
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", has_breached)
        .get("/robots.txt", |_, _| Response::ok(ROBOTS_TXT))
        .get("/img.png", |_, _| {
            Response::from_body(ResponseBody::Body(IMG_PNG.to_vec())).map(|x| {
                let mut headers = Headers::new();
                headers.set("content-type", "image/png").unwrap();
                x.with_headers(headers)
            })
        })
        .run(req, env)
        .await
}

#[inline]
fn has_breached<D>(_req: Request, _ctx: RouteContext<D>) -> Result<Response> {
    let date = Utc::now();

    let data = if date.month() == 11 {
        TemplateData::new("Yes".into())
    } else if date.month() == 12 {
        if date.day() <= 25 {
            TemplateData::new("Yes".into())
        } else {
            TemplateData::new("Attempting Recontainment".into())
        }
    } else if date.month() == 1 {
        TemplateData::new("Recontainment Successful, preforming clean up".into())
    } else {
        TemplateData::new("No".into())
    };

    let data = data;
    let mut page = handlebars::Handlebars::new()
        .render_template(include_str!("../content/index.hbs"), &data)
        .unwrap();

    if !page.starts_with("<!DOCTYPE html>") {
        page = format!("<!DOCTYPE html>{page}");
    }

    Response::from_html(page)
}
