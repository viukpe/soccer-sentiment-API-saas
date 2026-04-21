use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<Connection>>;

const VALID_SORT_FIELDS: &[&str] = &[
    "overall_score",
    "performance_score",
    "management_score",
    "transfers_score",
    "atmosphere_score",
];

#[derive(Deserialize)]
struct SentimentsParams {
    league: Option<String>,
    season: Option<String>,
    label: Option<String>,
    team: Option<String>,
    topic: Option<String>,
    sort: Option<String>,
    order: Option<String>,
}

fn error_response(code: u16, message: &str) -> Response {
    let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(json!({ "error": { "code": code, "message": message } }))).into_response()
}

fn team_row_to_value(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    let key_topics: String = row.get(13)?;
    let fan_voice_summary: String = row.get(14)?;
    let positive_highlights: String = row.get(15)?;
    let negative_highlights: String = row.get(16)?;
    Ok(json!({
        "id": row.get::<_, i64>(0)?,
        "team_id": row.get::<_, String>(1)?,
        "team_name": row.get::<_, String>(2)?,
        "league_id": row.get::<_, String>(3)?,
        "league_name": row.get::<_, String>(4)?,
        "season": row.get::<_, String>(5)?,
        "snapshot_date": row.get::<_, String>(6)?,
        "overall_score": row.get::<_, f64>(7)?,
        "overall_label": row.get::<_, String>(8)?,
        "scores": {
            "performance": row.get::<_, f64>(9)?,
            "management": row.get::<_, f64>(10)?,
            "transfers": row.get::<_, f64>(11)?,
            "atmosphere": row.get::<_, f64>(12)?,
        },
        "key_topics": serde_json::from_str::<Value>(&key_topics).unwrap_or(json!([])),
        "fan_voice_summary": fan_voice_summary,
        "positive_highlights": serde_json::from_str::<Value>(&positive_highlights).unwrap_or(json!([])),
        "negative_highlights": serde_json::from_str::<Value>(&negative_highlights).unwrap_or(json!([])),
    }))
}

async fn get_sentiments(
    State(db): State<Db>,
    Query(params): Query<SentimentsParams>,
) -> Response {
    if let Some(ref sort) = params.sort {
        if !VALID_SORT_FIELDS.contains(&sort.as_str()) {
            return error_response(
                400,
                &format!(
                    "Invalid value for 'sort'. Allowed values: {}.",
                    VALID_SORT_FIELDS.join(", ")
                ),
            );
        }
    }
    if let Some(ref order) = params.order {
        if order != "asc" && order != "desc" {
            return error_response(400, "Invalid value for 'order'. Allowed values: asc, desc.");
        }
    }

    let conn = db.lock().unwrap();

    let mut where_clauses: Vec<String> = vec![];
    let mut sql_params: Vec<String> = vec![];

    if let Some(ref v) = params.league {
        where_clauses.push("league_id = ?".to_string());
        sql_params.push(v.clone());
    }
    if let Some(ref v) = params.season {
        where_clauses.push("season = ?".to_string());
        sql_params.push(v.clone());
    }
    if let Some(ref v) = params.label {
        where_clauses.push("overall_label = ?".to_string());
        sql_params.push(v.clone());
    }
    if let Some(ref v) = params.team {
        where_clauses.push("team_name LIKE ?".to_string());
        sql_params.push(format!("%{}%", v));
    }
    if let Some(ref v) = params.topic {
        where_clauses.push("key_topics LIKE ?".to_string());
        sql_params.push(format!("%{}%", v));
    }

    let mut sql = "SELECT id, team_id, team_name, league_id, league_name, season, snapshot_date, \
                   overall_score, overall_label, performance_score, management_score, transfers_score, \
                   atmosphere_score, key_topics, fan_voice_summary, positive_highlights, negative_highlights \
                   FROM team_sentiment"
        .to_string();

    if !where_clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_clauses.join(" AND "));
    }

    if let Some(ref sort_field) = params.sort {
        let order = params.order.as_deref().unwrap_or("ASC").to_uppercase();
        sql.push_str(&format!(" ORDER BY {} {}", sort_field, order));
    }

    let result = (|| -> Result<Vec<Value>, rusqlite::Error> {
        let mut stmt = conn.prepare(&sql)?;
        let rows =
            stmt.query_map(rusqlite::params_from_iter(sql_params.iter()), team_row_to_value)?;
        rows.collect()
    })();

    match result {
        Err(e) => error_response(500, &e.to_string()),
        Ok(data) if data.is_empty() => {
            error_response(404, "No data found matching the applied filters.")
        }
        Ok(data) => {
            let mut filters = serde_json::Map::new();
            if let Some(ref v) = params.league {
                filters.insert("league".to_string(), json!(v));
            }
            if let Some(ref v) = params.season {
                filters.insert("season".to_string(), json!(v));
            }
            if let Some(ref v) = params.label {
                filters.insert("label".to_string(), json!(v));
            }
            if let Some(ref v) = params.team {
                filters.insert("team".to_string(), json!(v));
            }
            if let Some(ref v) = params.topic {
                filters.insert("topic".to_string(), json!(v));
            }
            if let Some(ref v) = params.sort {
                filters.insert("sort".to_string(), json!(v));
            }
            if let Some(ref v) = params.order {
                filters.insert("order".to_string(), json!(v));
            }
            (
                StatusCode::OK,
                Json(json!({
                    "count": data.len(),
                    "filters_applied": filters,
                    "data": data,
                })),
            )
                .into_response()
        }
    }
}

async fn get_leagues(State(db): State<Db>) -> Response {
    let conn = db.lock().unwrap();
    let result = (|| -> Result<Vec<Value>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT league_id, league_name, COUNT(*) as team_count \
             FROM team_sentiment \
             GROUP BY league_id, league_name \
             ORDER BY team_count DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(json!({
                "league_id": row.get::<_, String>(0)?,
                "league_name": row.get::<_, String>(1)?,
                "team_count": row.get::<_, i64>(2)?,
            }))
        })?;
        rows.collect()
    })();
    match result {
        Err(e) => error_response(500, &e.to_string()),
        Ok(data) => (
            StatusCode::OK,
            Json(json!({ "count": data.len(), "data": data })),
        )
            .into_response(),
    }
}

async fn get_seasons(State(db): State<Db>) -> Response {
    let conn = db.lock().unwrap();
    let result = (|| -> Result<Vec<Value>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT season, COUNT(*) as snapshot_count, MAX(snapshot_date) as latest_snapshot_date \
             FROM team_sentiment \
             GROUP BY season \
             ORDER BY latest_snapshot_date DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(json!({
                "season": row.get::<_, String>(0)?,
                "snapshot_count": row.get::<_, i64>(1)?,
                "latest_snapshot_date": row.get::<_, String>(2)?,
            }))
        })?;
        rows.collect()
    })();
    match result {
        Err(e) => error_response(500, &e.to_string()),
        Ok(data) => (
            StatusCode::OK,
            Json(json!({ "count": data.len(), "data": data })),
        )
            .into_response(),
    }
}

fn create_app(db: Db) -> Router {
    Router::new()
        .nest(
            "/v1",
            Router::new()
                .route("/sentiments", get(get_sentiments))
                .route("/leagues", get(get_leagues))
                .route("/seasons", get(get_seasons)),
        )
        .with_state(db)
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("data/sentiment.db").expect("Failed to open database");
    let state: Db = Arc::new(Mutex::new(conn));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, create_app(state)).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn setup_test_db() -> Db {
        let conn = Connection::open(":memory:").unwrap();
        conn.execute_batch(
            "CREATE TABLE team_sentiment (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                team_id TEXT NOT NULL, team_name TEXT NOT NULL,
                league_id TEXT NOT NULL, league_name TEXT NOT NULL,
                season TEXT NOT NULL, snapshot_date TEXT NOT NULL,
                overall_score FLOAT, overall_label TEXT,
                performance_score FLOAT, management_score FLOAT,
                transfers_score FLOAT, atmosphere_score FLOAT,
                key_topics TEXT, fan_voice_summary TEXT,
                positive_highlights TEXT, negative_highlights TEXT
            );
            INSERT INTO team_sentiment VALUES (1,'arsenal','Arsenal FC','premier-league','Premier League','2025/26','2026-04-15',72.5,'Positive',78.0,70.0,65.0,80.0,'[\"title race\",\"Saka\"]','Arsenal fans are optimistic.','[\"Saka form\"]','[\"Injury concerns\"]');
            INSERT INTO team_sentiment VALUES (2,'real-madrid','Real Madrid CF','la-liga','La Liga','2025/26','2026-04-15',88.5,'Very Positive',90.0,87.0,85.0,91.0,'[\"Vinicius\",\"Champions League\"]','Real fans are euphoric.','[\"Vinicius form\"]','[\"Defensive injuries\"]');
            INSERT INTO team_sentiment VALUES (3,'chelsea','Chelsea FC','premier-league','Premier League','2025/26','2026-04-15',31.0,'Negative',35.0,28.0,40.0,38.0,'[\"ownership\",\"inconsistency\"]','Chelsea fans are frustrated.','[\"Young signings\"]','[\"No identity\"]');",
        )
        .unwrap();
        Arc::new(Mutex::new(conn))
    }

    async fn body_json(res: axum::response::Response) -> Value {
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn req(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn test_sentiments_returns_all() {
        let res = create_app(setup_test_db()).oneshot(req("/v1/sentiments")).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 3);
    }

    #[tokio::test]
    async fn test_sentiments_filter_by_league() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?league=premier-league"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 2);
    }

    #[tokio::test]
    async fn test_sentiments_filter_by_topic() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?topic=Champions+League"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 1);
        assert_eq!(json["data"][0]["team_id"], "real-madrid");
    }

    #[tokio::test]
    async fn test_sentiments_filter_by_label() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?label=Negative"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 1);
        assert_eq!(json["data"][0]["team_id"], "chelsea");
    }

    #[tokio::test]
    async fn test_sentiments_sort_desc() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?sort=overall_score&order=desc"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        let data = json["data"].as_array().unwrap();
        assert_eq!(data[0]["team_id"], "real-madrid");
        assert_eq!(data[2]["team_id"], "chelsea");
    }

    #[tokio::test]
    async fn test_sentiments_not_found() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?league=nonexistent"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let json = body_json(res).await;
        assert_eq!(json["error"]["code"], 404);
    }

    #[tokio::test]
    async fn test_sentiments_invalid_sort_returns_400() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?sort=fake_field"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let json = body_json(res).await;
        assert_eq!(json["error"]["code"], 400);
    }

    #[tokio::test]
    async fn test_sentiments_invalid_order_returns_400() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?order=sideways"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_leagues_returns_aggregated() {
        let res = create_app(setup_test_db()).oneshot(req("/v1/leagues")).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 2);
        let pl = json["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|l| l["league_id"] == "premier-league")
            .unwrap()
            .clone();
        assert_eq!(pl["team_count"], 2);
    }

    #[tokio::test]
    async fn test_seasons_returns_aggregated() {
        let res = create_app(setup_test_db()).oneshot(req("/v1/seasons")).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        assert_eq!(json["count"], 1);
        assert_eq!(json["data"][0]["season"], "2025/26");
        assert_eq!(json["data"][0]["snapshot_count"], 3);
    }

    #[tokio::test]
    async fn test_sentiments_response_shape() {
        let res = create_app(setup_test_db())
            .oneshot(req("/v1/sentiments?team=Arsenal"))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let json = body_json(res).await;
        let record = &json["data"][0];
        assert!(record["scores"]["performance"].is_number());
        assert!(record["scores"]["management"].is_number());
        assert!(record["scores"]["transfers"].is_number());
        assert!(record["scores"]["atmosphere"].is_number());
        assert!(record["key_topics"].is_array());
        assert!(record["positive_highlights"].is_array());
        assert!(record["negative_highlights"].is_array());
    }
}
