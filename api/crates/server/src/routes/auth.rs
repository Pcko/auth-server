use crate::dto::auth_dto::{AuthMeResponseDTO, LoginDTO};
use crate::dto::register_dto::RegisterDTO;
use crate::dto::user_dto::UserResponseDTO;
use crate::errors::api_error::ApiError;
use crate::errors::error_body::{DocumentedApiError, ErrorBody, documented};
use crate::middleware::request_info_extractor::ExtractRequestInfo;
use crate::middleware::user_extractor::UserExtractor;
use crate::state::AppState;
use aide::NoApi;
use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use secrecy::ExposeSecret;
use tower_cookies::cookie::SameSite;
use tower_cookies::cookie::time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

type JsonResult<T> = Result<Json<T>, DocumentedApiError>;
type StatusResult = Result<StatusCode, DocumentedApiError>;

async fn register(State(state): State<AppState>, Json(dto): Json<RegisterDTO>) -> StatusResult {
    state
        .auth_service
        .register(dto.email, dto.username, dto.password)
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    Ok(StatusCode::CREATED)
}

async fn login(
    State(state): State<AppState>,
    NoApi(request_info): NoApi<ExtractRequestInfo>,
    NoApi(cookies): NoApi<Cookies>,
    Json(dto): Json<LoginDTO>,
) -> JsonResult<UserResponseDTO> {
    info!("login handler entered");
    let result = state
        .auth_service
        .login(
            dto.email,
            dto.password,
            request_info.into(),
            state.config.access_secret.as_ref(),
            state.config.refresh_secret.as_ref(),
            &state.config.audience,
            &state.config.issuer,
        )
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let mut session = Cookie::new("accessToken", result.access_token);
    configure_cookie(&state, result.access_expires_at, &mut session);

    let mut refresh = Cookie::new(
        "refreshToken",
        result.refresh_token.expose_secret().to_owned(),
    );
    configure_cookie(&state, result.refresh_expires_at, &mut refresh);

    cookies.add(session);
    cookies.add(refresh);

    Ok(Json(UserResponseDTO::from(result.user)))
}

async fn logout(State(state): State<AppState>, NoApi(cookies): NoApi<Cookies>) -> StatusCode {
    let revoke_result = state
        .auth_service
        .logout(
            cookies.get("accessToken").map(|c| c.value().to_string()),
            state.config.access_secret.as_ref(),
            cookies.get("refreshToken").map(|c| c.value().to_string()),
            state.config.refresh_secret.as_ref(),
        )
        .await;

    let mut access_token_removal = Cookie::new("accessToken", "");
    remove_cookie(&state, &mut access_token_removal);
    cookies.remove(access_token_removal);

    let mut refresh_token_removal = Cookie::new("refreshToken", "");
    remove_cookie(&state, &mut refresh_token_removal);
    cookies.remove(refresh_token_removal);

    if let Err(err) = revoke_result {
        tracing::error!(error = %err, "failed to revoke session during logout");
    }

    StatusCode::NO_CONTENT
}

async fn authenticate(
    NoApi(UserExtractor { user }): NoApi<UserExtractor>,
) -> JsonResult<UserResponseDTO> {
    Ok(Json(user.into()))
}

async fn refresh(
    State(state): State<AppState>,
    NoApi(request_info): NoApi<ExtractRequestInfo>,
    NoApi(cookies): NoApi<Cookies>,
) -> StatusResult {
    let refresh_cookie = cookies
        .get("refreshToken")
        .ok_or_else(|| documented(ApiError::Unauthorized("Unauthorized".to_string())))?;

    let result = state
        .auth_service
        .refresh_token(
            &state.config.audience,
            refresh_cookie.value(),
            state.config.refresh_secret.as_ref(),
            state.config.access_secret.as_ref(),
            &state.config.issuer,
        )
        .await
        .map_err(ApiError::from)
        .map_err(documented)?;

    let mut access_cookie = Cookie::new("accessToken", result.access_token);
    configure_cookie(&state, result.access_expires_at, &mut access_cookie);
    cookies.add(access_cookie);

    let mut refresh_cookie = Cookie::new(
        "refreshToken",
        result.refresh_token.expose_secret().to_owned(),
    );
    configure_cookie(&state, result.refresh_expires_at, &mut refresh_cookie);
    cookies.add(refresh_cookie);

    Ok(StatusCode::OK)
}

fn configure_cookie(state: &AppState, expires_at: OffsetDateTime, cookie: &mut Cookie) {
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(!state.config.is_dev);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(expires_at);
}

fn remove_cookie(state: &AppState, cookie_to_remove: &mut Cookie) {
    cookie_to_remove.set_path("/");
    cookie_to_remove.set_http_only(true);
    cookie_to_remove.set_same_site(SameSite::Lax);
    cookie_to_remove.set_secure(!state.config.is_dev);
    cookie_to_remove.make_removal();
}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/register",
            post_with(register, |op| {
                op.description("Create a new user account.")
                    .response::<201, ()>()
                    .response::<409, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/login",
            post_with(login, |op| {
                op.description("Authenticate a user and set access and refresh cookies.")
                    .response::<200, Json<UserResponseDTO>>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/logout",
            post_with(logout, |op| {
                op.description("Invalidate the active session and clear auth cookies.")
                    .security_requirement("accessCookie")
                    .security_requirement("refreshCookie")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/me",
            get_with(authenticate, |op| {
                op.description(
                    "Validate the current access token and return the authenticated user id.",
                )
                .security_requirement("accessCookie")
                .response::<200, Json<UserResponseDTO>>()
                .response::<401, Json<ErrorBody>>()
            }),
        )
        .api_route(
            "/refresh",
            post_with(refresh, |op| {
                op.description("Refresh the current access token using the refresh cookie.")
                    .security_requirement("refreshCookie")
                    .response::<200, ()>()
                    .response::<401, Json<ErrorBody>>()
                    .response::<500, Json<ErrorBody>>()
            }),
        )
}
