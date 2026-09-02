use sdkwork_api_mahjong_assembly::assemble_api_router;
use sdkwork_iam_web_adapter::{
    build_web_framework_builder, iam_web_request_context_resolver_from_env,
};
use sdkwork_utils_rust::optional::default_if_blank;
use sdkwork_web_bootstrap::{ApiModuleRegistry, ComposedApiAssembly, infra_public_path_prefixes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bind_address = default_if_blank(
        std::env::var("MAHJONG_API_BIND")
            .ok()
            .or_else(|| std::env::var("SDKWORK_MAHJONG_APPLICATION_PUBLIC_INGRESS_BIND").ok())
            .as_deref(),
        "127.0.0.1:8097",
    );

    let assembly = assemble_api_router()
        .await
        .expect("mahjong API assembly bootstrap failed");
    let framework = build_web_framework_builder(
        iam_web_request_context_resolver_from_env().await,
        assembly.route_manifest.clone(),
        infra_public_path_prefixes(),
    );
    let mut module_registry = ApiModuleRegistry::new();
    module_registry.add_modules(vec![assembly]);
    let hosted = module_registry
        .try_compose("SDKWork Mahjong API")
        .expect("mahjong API composition failed")
        .into_hosted(framework);
    let app = hosted
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_MAHJONG_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        ));
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind mahjong standalone-gateway listener failed");
    tracing::info!("sdkwork-api-mahjong-standalone-gateway listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve mahjong standalone-gateway failed");
}
