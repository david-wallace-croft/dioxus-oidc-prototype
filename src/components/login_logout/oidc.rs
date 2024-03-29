use crate::log::LogId;
use crate::storage::StorageKey;
use crate::{storage, window};
use ::oauth2::{CodeTokenRequest, PkceCodeChallenge, PkceCodeVerifier};
use ::openidconnect::{
  core::{
    CoreClient, CoreErrorResponseType, CoreIdToken, CoreResponseType,
    CoreTokenResponse,
  },
  reqwest::async_http_client,
  url::Url,
  AuthenticationFlow, AuthorizationCode, ClaimsVerificationError, ClientId,
  CsrfToken, IssuerUrl, LogoutRequest, Nonce, ProviderMetadataWithLogout,
  RedirectUrl, RefreshToken, RequestTokenError, StandardErrorResponse,
};
use ::serde::{Deserialize, Serialize};

/// State that holds the nonce and authorization url and the nonce generated to
/// log in an user
#[derive(Clone, Deserialize, Serialize, Default)]
pub struct AuthRequestState {
  pub auth_request: Option<AuthRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequest {
  pub nonce: Nonce,
  pub authorize_url: String,
}

/// State the tokens returned once the user is authenticated
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AuthTokenState {
  /// Token used to identify the user
  pub id_token: Option<CoreIdToken>,
  /// Token used to refresh the tokens if they expire
  pub refresh_token: Option<RefreshToken>,
}

pub fn email(
  client: CoreClient,
  id_token: CoreIdToken,
  nonce: Nonce,
) -> Result<String, ClaimsVerificationError> {
  match id_token.claims(&client.id_token_verifier(), &nonce) {
    Ok(claims) => Ok(claims.clone().email().unwrap().to_string()),
    Err(error) => Err(error),
  }
}

pub fn authorize_url(
  client: CoreClient,
  csrf_token: CsrfToken,
) -> AuthRequest {
  let (pkce_challenge, pkce_verifier): (PkceCodeChallenge, PkceCodeVerifier) =
    PkceCodeChallenge::new_random_sha256();
  let pkce_verifier_secret: &str = pkce_verifier.secret();
  log::info!("authorize_url() pkce_verifier: {pkce_verifier_secret}");
  // TODO: What if result is Err?
  let _result = storage::set(StorageKey::PkceVerifier, pkce_verifier_secret);
  // TODO: What about the csrf state?
  let (authorize_url, _csrf_state, nonce) = client
    .authorize_url(
      AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
      || csrf_token,
      Nonce::new_random,
    )
    // TODO: Do I need to add the openid scope?
    // .add_scope(openidconnect::Scope::new("email".to_string()))
    // .add_scope(openidconnect::Scope::new("profile".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();
  AuthRequest {
    authorize_url: authorize_url.to_string(),
    nonce,
  }
}

pub async fn init_provider_metadata(
) -> Result<ProviderMetadataWithLogout, super::errors::Error> {
  let issuer_url =
    IssuerUrl::new(super::constants::DIOXUS_FRONT_ISSUER_URL.to_string())?;
  Ok(
    ProviderMetadataWithLogout::discover_async(issuer_url, async_http_client)
      .await?,
  )
}

pub async fn init_oidc_client(
) -> Result<(ClientId, CoreClient), super::errors::Error> {
  let client_id =
    ClientId::new(super::constants::DIOXUS_FRONT_CLIENT_ID.to_string());

  let provider_metadata = init_provider_metadata().await?;

  log::debug!("{} provider_metadata: {provider_metadata:#?}", LogId::L039);

  let client_secret = None;

  let Some(origin) = window::get_origin() else {
    return Err(super::errors::Error::WindowOrigin);
  };

  let redirect_url_string: String = format!(
    "{}/callback",
    // super::constants::DIOXUS_FRONT_URL
    origin,
  );

  let redirect_url = RedirectUrl::new(redirect_url_string)?;

  Ok((
    client_id.clone(),
    CoreClient::from_provider_metadata(
      provider_metadata,
      client_id,
      client_secret,
    )
    .set_redirect_uri(redirect_url),
  ))
}

pub async fn token_response(
  authorization_code_string: String,
  client: &CoreClient,
  pkce_verifier_string: String,
) -> Result<CoreTokenResponse, super::errors::Error> {
  let authorization_code = AuthorizationCode::new(authorization_code_string);
  let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_string);
  let code_token_request: CodeTokenRequest<_, _, _> = client
    .exchange_code(authorization_code)
    .set_pkce_verifier(pkce_verifier)
    // TODO: Is this openid scope necessary?
    .add_extra_param("scope", "openid");
  let result: CoreTokenResponse =
    code_token_request.request_async(async_http_client).await?;
  Ok(result)
}

pub async fn exchange_refresh_token(
  oidc_client: CoreClient,
  refresh_token: RefreshToken,
) -> Result<
  CoreTokenResponse,
  RequestTokenError<
    openidconnect::reqwest::Error<reqwest::Error>,
    StandardErrorResponse<CoreErrorResponseType>,
  >,
> {
  oidc_client
    .exchange_refresh_token(&refresh_token)
    .request_async(async_http_client)
    .await
}

pub async fn log_out_url(
  id_token_hint: CoreIdToken
) -> Result<Url, super::errors::Error> {
  let provider_metadata = init_provider_metadata().await?;
  let end_session_url = provider_metadata
    .additional_metadata()
    .clone()
    .end_session_endpoint
    .unwrap();
  let logout_request: LogoutRequest = LogoutRequest::from(end_session_url);
  Ok(
    logout_request
      .set_client_id(ClientId::new(
        super::constants::DIOXUS_FRONT_CLIENT_ID.to_string(),
      ))
      .set_id_token_hint(&id_token_hint)
      .http_get_url(),
  )
}
