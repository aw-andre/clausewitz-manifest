use super::*;
use axum::{extract::Path, response::IntoResponse};
use tracing::info;

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate {
    pub game: String,
}

pub async fn form(Path(game): Path<String>) -> impl IntoResponse {
    info!("getting form for {}", game);
    let template = FormTemplate { game };
    HtmlTemplate(template)
}
