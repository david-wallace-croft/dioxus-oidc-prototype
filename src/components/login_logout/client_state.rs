use super::props::client::ClientProps;
use crate::log::LogId;
use ::dioxus::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ClientState {
  pub oidc_client: Option<ClientProps>,
}

impl ClientState {
  // TODO: consolidate copy and paste of this function from login-logout component
  pub fn read_client_props_from_shared_state(
    use_shared_state_client_state: UseSharedState<ClientState>
  ) -> Option<ClientProps> {
    log::info!(
      "{} Reading client properties from shared state...",
      LogId::L002
    );
    let client_state_ref: Ref<'_, ClientState> =
      use_shared_state_client_state.read();
    let client_props_option_ref: &Option<ClientProps> =
      &client_state_ref.oidc_client;
    if client_props_option_ref.is_none() {
      return None;
    }
    let client_props_option: Option<&ClientProps> =
      client_props_option_ref.as_ref();
    log::info!(
      "{} Client properties loaded from shared state.",
      LogId::L003
    );
    let client_props: &ClientProps = client_props_option.unwrap();
    Some(client_props.clone())
  }
}
