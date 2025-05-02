use std::vec::IntoIter;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::Query;
use serde::Deserialize;
use sqlx::{Pool, Postgres, query};
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

#[derive(Deserialize)]
pub struct TreeParams {
    search_term: Option<String>,

    #[serde(default)]
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

    info!(
        "getting tree for search_term: {}, search_type: {:#?}",
        search_term, search_type
    );

    let mut all_nodes = Vec::new();
    if search_type.contains(&"value".to_string()) {
        let rows = query!(
            "
            WITH RECURSIVE parent_chain AS (
                SELECT
                f.primary_id,
                f.key,
                f.value,
                f.parent_id,
                f.primary_id AS start_id,
                RANK() OVER (ORDER BY f.value::bytea ASC NULLS FIRST) AS rank,
                0 AS depth
                FROM gamefiles f
                WHERE f.game = $1 and f.value = $2
                UNION ALL

                SELECT
                    f.primary_id,
                    f.key,
                    f.value,
                    f.parent_id,
                    pc.start_id,
                    pc.rank,
                    pc.depth - 1 AS depth
                FROM gamefiles f
                JOIN parent_chain pc
                  ON f.primary_id = pc.parent_id
            )

            SELECT primary_id, key, value, parent_id
            FROM parent_chain
            ORDER BY rank, start_id, depth
            ",
            game,
            search_term
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        for row in rows {
            all_nodes.push(Node {
                primary_id: row.primary_id.unwrap(),
                group_id: None,
                key: row.key.unwrap(),
                value: row.value,
                parent_id: row.parent_id,
                child_id: None,
                displayed_child: None,
            });
        }
    }

    if search_type.contains(&"key".to_string()) {
        let rows = query!(
            "
            WITH RECURSIVE parent_chain AS (
                SELECT
                f.primary_id,
                f.key,
                f.value,
                f.parent_id,
                f.primary_id AS start_id,
                RANK() OVER (ORDER BY f.value::bytea ASC NULLS FIRST) AS rank,
                0 AS depth
                FROM gamefiles f
                WHERE f.game = $1 and f.key = $2
                UNION ALL

                SELECT
                    f.primary_id,
                    f.key,
                    f.value,
                    f.parent_id,
                    pc.start_id,
                    pc.rank,
                    pc.depth - 1 AS depth
                FROM gamefiles f
                JOIN parent_chain pc
                  ON f.primary_id = pc.parent_id
            )

            SELECT primary_id, key, value, parent_id
            FROM parent_chain
            ORDER BY rank, start_id, depth
            ",
            game,
            search_term
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        for row in rows {
            all_nodes.push(Node {
                primary_id: row.primary_id.unwrap(),
                group_id: None,
                key: row.key.unwrap(),
                value: row.value,
                parent_id: row.parent_id,
                child_id: None,
                displayed_child: None,
            });
        }
    }

    async fn make_parent_hierarchy(mut nodes: Vec<Node>) -> Vec<Node> {
        let mut hierarchy = Vec::new();

        let mut child = None;
        let mut parent: Option<Node>;
        loop {
            parent = nodes.pop();
            if parent.is_none() {
                break;
            }
            let mut parent = parent.unwrap();
            parent.displayed_child = child;
            match parent.parent_id {
                None => {
                    hierarchy.push(parent);
                    child = None;
                }
                Some(_) => {
                    child = Some(Box::new(parent));
                }
            }
        }

        hierarchy
    }

    info!("preparing hierarchy");
    // Return template
    let template = TreeTemplate {
        nodes: make_parent_hierarchy(all_nodes).await,
    };
    info!("preparing HTML");
    HtmlTemplate(template)
}
