use ::dioxus::prelude::*;
use ::dioxus_router::routable::FromQuery;
use ::form_urlencoded::Parse;
use ::std::borrow::Cow;
use ::std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CallbackQuerySegments {
  pub placeholder: String,
}

impl Display for CallbackQuerySegments {
  fn fmt(
    &self,
    formatter: &mut Formatter<'_>,
  ) -> Result {
    write!(formatter, "placeholder={}", self.placeholder)
  }
}

impl FromQuery for CallbackQuerySegments {
  fn from_query(query: &str) -> Self {
    let mut placeholder_option: Option<String> = None;
    let input: &[u8] = query.as_bytes();
    let pairs: Parse<'_> = ::form_urlencoded::parse(input);
    pairs.for_each(|(key, value): (Cow<'_, str>, Cow<'_, str>)| {
      if key == "placeholder" {
        placeholder_option = Some(value.clone().into());
      }
    });
    Self {
      placeholder: placeholder_option.unwrap_or_default(),
    }
  }
}

#[allow(non_snake_case)]
#[component]
pub fn Callback(
  cx: Scope,
  query_params: CallbackQuerySegments,
) -> Element {
  render! {
  main {
    class: "app-callback",
  h1 {
  "Callback"
  }
  p {
  "query_params: {query_params:?}"
  }
  }
  }
}
