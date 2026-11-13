use std::path::Path;
use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
struct RouteManifestFile {
    routes: Vec<RouteManifestRoute>,
}

#[derive(Deserialize)]
struct RouteManifestRoute {
    method: String,
    path: String,
    #[serde(rename = "operationId")]
    operation_id: String,
    tags: Vec<String>,
    auth: RouteManifestAuth,
    #[serde(default)]
    idempotent: bool,
}

#[derive(Deserialize)]
struct RouteManifestAuth {
    mode: String,
    #[serde(default)]
    permission: Option<String>,
}

fn main() {
    let manifest_path = Path::new(
        "../../sdks/_route-manifests/app-api/sdkwork-routes-mahjong-app-api.route-manifest.json",
    );
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let raw = fs::read_to_string(manifest_path)
        .unwrap_or_else(|error| panic!("read {} failed: {error}", manifest_path.display()));
    let manifest: RouteManifestFile = serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("parse {} failed: {error}", manifest_path.display()));
    let generated = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("routes.rs");
    fs::write(generated, render_routes(&manifest.routes)).expect("write generated match routes");
}

fn render_routes(routes: &[RouteManifestRoute]) -> String {
    let mut output = String::from(
        "// @generated from the canonical route manifest; do not edit.\nuse sdkwork_web_contract::{HttpMethod, HttpRoute, RouteAuth};\n\npub const MATCH_HTTP_ROUTES: &[HttpRoute] = &[\n",
    );
    for route in routes {
        let method = match route.method.as_str() {
            "GET" => "HttpMethod::Get",
            "POST" => "HttpMethod::Post",
            "PUT" => "HttpMethod::Put",
            "PATCH" => "HttpMethod::Patch",
            "DELETE" => "HttpMethod::Delete",
            other => panic!("unsupported HTTP method: {other}"),
        };
        let auth = match route.auth.mode.as_str() {
            "public" => "RouteAuth::Public",
            "dual-token" => "RouteAuth::DualToken",
            "api-key" => "RouteAuth::ApiKey",
            other => panic!("unsupported auth mode: {other}"),
        };
        let tag = route.tags.first().map(String::as_str).unwrap_or("mahjong");
        let permission = route
            .auth
            .permission
            .as_deref()
            .map(|permission| format!(".with_required_permission({permission:?})"))
            .unwrap_or_default();
        output.push_str(&format!(
            "    HttpRoute::new({method}, {:?}, {:?}, {:?}, {auth}){}{} ,\n",
            route.path,
            tag,
            route.operation_id,
            if route.idempotent {
                ".with_idempotent(true)"
            } else {
                ""
            },
            permission,
        ));
    }
    output.push_str("];\n");
    output
}
