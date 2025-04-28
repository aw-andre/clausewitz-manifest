use askama::Template;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use std::collections::HashMap;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
pub struct HtmlTemplate<T>(pub T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

pub struct Node {
    pub primary_id: u32,
    pub group_id: Option<u32>,
    pub key: String,
    pub value: Option<String>,
    pub parent_id: Option<u32>,
    pub child_id: Option<u32>,
}

#[derive(Template)]
#[template(path = "tree.html")]
pub struct TreeTemplate {
    pub valid: bool,
    pub game: String,
    pub nodes: Vec<Node>,
}

#[derive(Template)]
#[template(path = "open-node.html")]
pub struct OpenNodeTemplate {
    pub contents: Node,
}

#[derive(Template)]
#[template(path = "closed-node.html")]
pub struct ClosedNodeTemplate {
    pub contents: Node,
}

#[derive(Template)]
#[template(path = "oneshot-node.html")]
pub struct OneshotNodeTemplate {
    pub contents: Node,
}

pub async fn modifier_tree(Path(game): Path<String>) -> impl IntoResponse {
    let template = TreeTemplate {
        valid: match game.as_str() {
            "eu3" | "eu4" | "ck2" | "ck3" | "hoi3" | "vic2" | "vic3" | "imperator"
            | "stellaris" => true,
            _ => false,
        },
        game,
        nodes: vec![],
    };
    HtmlTemplate(template)
}
