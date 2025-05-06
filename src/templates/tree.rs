use super::*;
use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_extra::extract::Query;
use serde::Deserialize;
use sqlx::{Pool, Postgres, Row};
use std::collections::VecDeque;
use tracing::info;

#[derive(Deserialize)]
pub struct TreeParams {
    search_term: Option<String>,

    #[serde(default)]
    search_type: Vec<String>,

    start: i64,
    end: i64,
}

#[derive(Template)]
#[template(path = "tree.html")]
pub struct TreeTemplate {
    pub nodes: VecDeque<Node>,
}

pub async fn tree(
    Path(game): Path<String>,
    Query(params): Query<TreeParams>,
    State(pool): State<Pool<Postgres>>,
) -> impl IntoResponse {
    // Get matching rows
    let search_term = params.search_term.unwrap_or_default();
    let search_type = params.search_type;
    let start = params.start;
    let end = params.end;

    info!(
        "getting tree for search_term: {}, search_type: {:#?}, start {}, end, {}",
        search_term, search_type, start, end
    );

    let mut all_nodes = Vec::new();
    let mut where_clause = "";

    if search_type.contains(&"key".to_string()) && search_type.contains(&"value".to_string()) {
        where_clause = "f.key = $2 OR f.value = $2";
    } else if search_type.contains(&"key".to_string()) {
        where_clause = "f.key = $2";
    } else if search_type.contains(&"value".to_string()) {
        where_clause = "f.value = $2";
    }

    let query_str = format!(
        "
        WITH RECURSIVE parent_chain AS (
            SELECT * FROM (
                SELECT
                f.primary_id,
                f.key,
                f.value,
                f.parent_id,
                f.primary_id AS start_id,
                RANK() OVER (ORDER BY f.value::bytea DESC NULLS LAST, f.primary_id) AS rank,
                0 AS depth
                FROM gamefiles AS f
                WHERE f.game = $1 AND ({})
            ) AS base
            WHERE base.rank >= $3 AND base.rank < $4
            UNION ALL

            SELECT
                f.primary_id,
                f.key,
                f.value,
                f.parent_id,
                pc.start_id,
                pc.rank,
                pc.depth - 1 AS depth
            FROM gamefiles AS f
            JOIN parent_chain AS pc
              ON f.primary_id = pc.parent_id
        )

        SELECT primary_id, key, value, parent_id, rank
        FROM parent_chain
        ORDER BY rank, start_id, depth
        ",
        where_clause
    );

    let rows = sqlx::query(&query_str)
        .bind(game)
        .bind(search_term)
        .bind(start)
        .bind(start + STEP)
        .fetch_all(&pool)
        .await
        .unwrap();

    info!("pushing rows");
    for row in rows {
        all_nodes.push(Node {
            primary_id: row.get::<i32, _>("primary_id"),
            key: row.get::<String, _>("key"),
            value: row.get::<Option<String>, _>("value"),
            parent_id: row.get::<Option<i32>, _>("parent_id"),
            rank: row.get::<i64, _>("rank"),
            displayed_child: None,
        });
    }

    async fn make_parent_hierarchy(mut nodes: Vec<Node>) -> VecDeque<Node> {
        let mut hierarchy = VecDeque::new();

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
                    hierarchy.push_front(parent);
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
