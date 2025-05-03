use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
pub mod form;
pub mod index;
pub mod tree;

const STEP: i64 = 10;

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

#[derive(Debug, Template)]
#[template(path = "node.html", ext = "html", escape = "none")]
pub struct Node {
    pub primary_id: i32,
    pub group_id: Option<i32>,
    pub key: String,
    pub value: Option<String>,
    pub parent_id: Option<i32>,
    pub child_id: Option<i32>,
    pub displayed_child: Option<Box<Node>>,
}
