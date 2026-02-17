pub mod query;
pub mod evaluate;
pub mod response;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::env;

use query::UpdateQuery;
use evaluate::evaluate_rules;
use response::construct_xml_response;
pub async fn handle_update(
    State(pool): State<MySqlPool>,
    Path((product, version, build_id, build_target, locale, channel, os_version, system_capabilities, distribution, dist_version)):
        Path<(String, String, String, String, String, String, String, String, String, String)>,
    Query(query_params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    // Get User-Agent header
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Build query object
    let query = UpdateQuery::from_path_and_query(
        product,
        version,
        build_id,
        build_target,
        locale,
        channel,
        os_version,
        system_capabilities,
        distribution,
        dist_version,
        query_params,
        user_agent,
    );

    // Evaluate rules
    let result = match evaluate_rules(&pool, &query).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Error evaluating rules: {:?}", e);
            return empty_response();
        }
    };

    // Generate XML
    let xml = if let Some(blob) = result.blob {
        if let Some(xml_blob) = blob.as_xml_blob() {
            match xml_blob.get_xml(&query) {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!("Error generating XML: {:?}", e);
                    return empty_response();
                }
            }
        } else {
            return empty_response();
        }
    } else {
        return empty_response();
    };

    // Construct response with headers
    let xml = construct_xml_response(xml);

    let cache_control = env::var("CACHE_CONTROL")
        .unwrap_or_else(|_| "public, max-age=90".to_string());

    let mut response_headers = vec![
        ("Content-Type", "text/xml".to_string()),
        ("Cache-Control", cache_control),
    ];

    if let Some(rule_id) = result.rule_id {
        response_headers.push(("X-Rule-ID", rule_id.to_string()));
    }

    if let Some(data_version) = result.rule_data_version {
        response_headers.push(("X-Rule-Data-Version", data_version.to_string()));
    }

    let mut resp = (StatusCode::OK, xml).into_response();

    let headers_map = resp.headers_mut();
    for (key, value) in response_headers {
        if let Ok(header_value) = value.parse() {
            headers_map.insert(key, header_value);
        }
    }

    resp
}

fn empty_response() -> Response {
    let xml = "<?xml version=\"1.0\"?>\n<updates></updates>";

    (
        StatusCode::OK,
        [("Content-Type", "text/xml")],
        xml,
    ).into_response()
}
