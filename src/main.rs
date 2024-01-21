use dioxus_oidc_prototype::launch;

#[cfg(feature = "hydrate")]
fn main() {
  dioxus_web::launch_with_props(
    dioxus_fullstack::router::RouteWithCfg::<dioxus_oidc_protype::route::Route>,
    dioxus_fullstack::prelude::get_root_props_from_document()
      .expect("Failed to get root props from document"),
    dioxus_web::Config::default().hydrate(true),
  );
}

#[cfg(not(feature = "hydrate"))]
fn main() {
  launch();
}
