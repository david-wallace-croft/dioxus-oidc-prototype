use ::dioxus_router::routable::*;
use ::form_urlencoded::Parse;
use ::serde::{Deserialize, Serialize};
use ::std::borrow::Cow;
use ::std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CallbackQuerySegments {
  pub code: String,
  pub state: String,
}

impl Display for CallbackQuerySegments {
  fn fmt(
    &self,
    formatter: &mut Formatter<'_>,
  ) -> fmt::Result {
    write!(formatter, "code={}&state={}", self.code, self.state)
  }
}

impl FromQuery for CallbackQuerySegments {
  fn from_query(query: &str) -> Self {
    let mut code = String::default();
    let mut state = String::default();
    let input: &[u8] = query.as_bytes();
    let pairs: Parse<'_> = ::form_urlencoded::parse(input);
    pairs.for_each(|(key, value): (Cow<'_, str>, Cow<'_, str>)| {
      if key == "code" {
        code = value.into();
      } else if key == "state" {
        state = value.into();
      }
    });
    Self {
      code,
      state,
    }
  }
}
