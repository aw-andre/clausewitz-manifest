use super::*;
use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::Query;
use serde::Deserialize;
use sqlx::{Pool, Postgres, query};
use tracing::info;

#[derive(Deserialize)]
pub struct ChildrenParams {
    parent_id: i64,
    displayed_child_id: Option<i64>,
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
    let displayed_child_id = params.displayed_child_id;

    info!("getting children for {}", parent_id);

    let mut nodes = Vec::new();
    if displayed_child_id.is_some() {
        let rows = query!(
            "
            SELECT primary_id, key, value, parent_id
            FROM gamefiles
            WHERE parent_id = $1 AND primary_id != $2
            ",
            parent_id,
            displayed_child_id.unwrap() as i32
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for row in rows {
            nodes.push(Node {
                primary_id: row.primary_id,
                key: row.key,
                value: row.value,
                parent_id: row.parent_id,
                displayed_child: None,
            });
        }
    } else {
        let rows = query!(
            "
            SELECT primary_id, key, value, parent_id
            FROM gamefiles
            WHERE parent_id = $1
            ",
            parent_id,
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for row in rows {
            nodes.push(Node {
                primary_id: row.primary_id,
                key: row.key,
                value: row.value,
                parent_id: row.parent_id,
                displayed_child: None,
            });
        }
    }

    let template = ChildrenTemplate { nodes };

    HtmlTemplate(template)
}
