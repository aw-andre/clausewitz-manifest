use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sqlx::{Pool, Postgres, query};

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
    pub primary_id: i32,
    pub group_id: Option<i32>,
    pub key: String,
    pub value: Option<String>,
    pub parent_id: Option<i32>,
    pub child_id: Option<i32>,
    pub displayed_child: Option<Box<Node>>,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate {
    pub game: String,
}

pub async fn form(Path(game): Path<String>) -> impl IntoResponse {
    let template = FormTemplate { game };
    HtmlTemplate(template)
}

#[derive(serde::Deserialize)]
pub struct TreeParams {
    search_term: Option<String>,
    search_type: Vec<String>,
}

#[derive(Template)]
#[template(path = "tree.html")]
pub struct TreeTemplate {
    pub nodes: Vec<Node>,
}

pub async fn tree(
    Path(game): Path<String>,
    Query(params): Query<TreeParams>,
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    // Get matching rows
    let search_term = params.search_term.unwrap_or_default();
    let search_type = params.search_type;

    let mut matching_nodes = Vec::new();
    if search_type.contains(&"key".to_string()) {
        let rows = query!(
            "SELECT * FROM gamefiles WHERE game = $1 AND key = $2",
            game,
            search_term
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for row in rows {
            matching_nodes.push(Node {
                primary_id: row.primary_id,
                group_id: row.group_id,
                key: row.key,
                value: row.value,
                parent_id: row.parent_id,
                child_id: row.child_id,
                displayed_child: None,
            })
        }
    }

    if search_type.contains(&"value".to_string()) {
        let rows = query!(
            "SELECT * FROM gamefiles WHERE game = $1 AND value = $2",
            game,
            search_term
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for row in rows {
            matching_nodes.push(Node {
                primary_id: row.primary_id,
                group_id: row.group_id,
                key: row.key,
                value: row.value,
                parent_id: row.parent_id,
                child_id: row.child_id,
                displayed_child: None,
            })
        }
    }

    // Create parent hierarchy
    async fn make_parent_hierarchy(current: Node, pool: Pool<Postgres>) -> Node {
        match current.parent_id {
            Some(parent_id) => {
                let parent_row = query!("SELECT * FROM gamefiles WHERE primary_id = $1", parent_id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                let parent_node = Node {
                    primary_id: parent_row.primary_id,
                    group_id: parent_row.group_id,
                    key: parent_row.key,
                    value: parent_row.value,
                    parent_id: parent_row.parent_id,
                    child_id: parent_row.child_id,
                    displayed_child: Some(Box::new(current)),
                };
                Box::pin(make_parent_hierarchy(parent_node, pool)).await
            }
            None => current,
        }
    }
    let mut displayed_nodes = Vec::new();
    for node in matching_nodes {
        displayed_nodes.push(make_parent_hierarchy(node, pool.clone()).await);
    }

    // Return template
    let template = TreeTemplate {
        nodes: displayed_nodes,
    };
    HtmlTemplate(template)
}

pub struct NodeTemplate {
    pub contents: Node,
}
