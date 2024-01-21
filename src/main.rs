#[cfg(feature = "hydrate")]
use ::dioxus_fullstack::prelude::*;
#[cfg(feature = "hydrate")]
use ::dioxus_fullstack::router::{FullstackRouterConfig, RouteWithCfg};
#[cfg(feature = "hydrate")]
use ::dioxus_web::Config;
#[cfg(feature = "hydrate")]
use dioxus_oidc_prototype::route::Route;

#[cfg(feature = "hydrate")]
fn main() {
  let root_properties: FullstackRouterConfig<Route> =
    get_root_props_from_document()
      .expect("Failed to get root properties from document");
  let config = Config::default().hydrate(true);
  ::dioxus_web::launch_with_props(
    RouteWithCfg::<Route>,
    root_properties,
    config,
  );
}

#[cfg(not(feature = "hydrate"))]
fn main() {
  dioxus_oidc_prototype::launch();
}
