use askama::Template;
use axum::response::IntoResponse;

use crate::templates::HtmlTemplate;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate;

pub async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

pub async fn test() -> &'static str {
    "Using API"
}
