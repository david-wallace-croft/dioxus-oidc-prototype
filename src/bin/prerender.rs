use ::dioxus_fullstack::prelude::*;
use ::dioxus_fullstack::router::FullstackRouterConfig;
use dioxus_oidc_prototype::route::Route;

const DIST: &str = "dist";

#[tokio::main]
async fn main() {
  let fullstack_router_config = FullstackRouterConfig::<Route>::default();
  let incremental_renderer_config =
    IncrementalRendererConfig::default().static_dir(DIST);
  let serve_config: ServeConfig<FullstackRouterConfig<Route>> =
    ServeConfigBuilder::new_with_router(fullstack_router_config)
      .assets_path(DIST)
      .incremental(incremental_renderer_config)
      .build();
  pre_cache_static_routes_with_props(&serve_config)
    .await
    .unwrap();
}
