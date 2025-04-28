use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tracing::info;

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
    pub id: String,
    pub label: String,
    pub value: Option<f64>,
    /// true if this node *may* have children (for lazy loading)
    pub has_children: bool,
}

#[derive(Template)]
#[template(path = "tree.html")]
pub struct ModifierTreeTemplate<'a> {
    pub modifier_key: &'a str,
    /// A Vec of “root paths” leading down to this modifier.
    /// Each path is a Vec<Node> from the top parent → … → the leaf modifier.
    pub paths: Vec<Vec<Node>>,
}

#[derive(Template)]
#[template(path = "node.html")]
pub struct TreeNodeTemplate {
    pub id: String,
    pub children: Vec<Node>,
}

pub async fn modifier_tree(game: &'static str) -> impl IntoResponse {
    info!("printing modifier_tree");
    let template = ModifierTreeTemplate {
        modifier_key: "",
        paths: vec![],
    };
    HtmlTemplate(template)
}
