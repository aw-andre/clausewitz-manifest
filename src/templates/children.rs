use super::*;
use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use sqlx::{Pool, Postgres, Row, query};
use tracing::info;

#[derive(Deserialize)]
pub struct ChildrenParams {
    parent_id: i64,
}

#[derive(Template)]
#[template(path = "children.html")]
pub struct ChildrenTemplate {
    pub nodes: Vec<Node>,
}

pub async fn children(
    Query(params): Query<ChildrenParams>,
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    let parent_id = params.parent_id as i32;

    info!("getting children for {}", parent_id);

    let rows = query!(
        "
        SELECT primary_id, key, value, parent_id
        FROM gamefiles
        WHERE parent_id = $1
        ",
        parent_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut nodes = Vec::new();
    for row in rows {
        nodes.push(Node {
            primary_id: row.primary_id,
            group_id: None,
            key: row.key,
            value: row.value,
            parent_id: row.parent_id,
            child_id: None,
            displayed_child: None,
        });
    }

    let template = ChildrenTemplate { nodes };

    HtmlTemplate(template)
}
