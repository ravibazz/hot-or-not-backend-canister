use std::borrow::Cow;

use candid::{candid_method, types::principal, CandidType};
use ic_cdk::api;
use serde::Deserialize;
use shared_utils::common::{
    types::http::{HeaderField, HttpRequest, HttpResponse},
    utils::{get_heap_memory_size, get_stable_memory_size},
};

fn get_path(url: &str) -> Option<&str> {
    url.split('?').next()
}

fn metrics() -> String {
    // Prometheus expects timestamps in ms. Time.now() returns ns.
    let timestamp = api::time() / 1000000;
    let canister_type = "post_cache";

    vec![
        format!(
            "cycle_balance{{type=\"{}\"}} {} {}",
            canister_type,
            api::canister_balance128(),
            timestamp
        )
        .as_str(),
        format!(
            "heap_size{{type=\"{}\"}} {} {}",
            canister_type,
            get_heap_memory_size(),
            timestamp
        )
        .as_str(),
        format!(
            "stable_size{{type=\"{}\"}} {} {}",
            canister_type,
            get_stable_memory_size(),
            timestamp
        )
        .as_str(),
    ]
    .connect("\n")
}

fn retrieve(path: &str) -> Option<Vec<u8>> {
    match path {
        "/metrics" => Some(metrics().as_bytes().to_vec()),
        _ => None,
    }
}

#[ic_cdk_macros::query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path = get_path(request.url.as_str()).unwrap_or("/");
    if let Some(bytes) = retrieve(path) {
        HttpResponse {
            status_code: 200,
            headers: vec![
                //HeaderField("Content-Encoding".to_string(), "gzip".to_string()),
                HeaderField("Content-Length".to_string(), format!("{}", bytes.len())),
                HeaderField("Cache-Control".to_string(), format!("max-age={}", 600)),
                HeaderField("Content-Type".to_string(), "text/plain".to_string()),
            ],
            body: bytes,
        }
    } else {
        HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: path.as_bytes().to_vec(),
        }
    }
}
