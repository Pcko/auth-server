use crate::state::AppState;
use aide::axum::ApiRouter;

//TODO invitation system
//  fn invite(State(state): State<AppState>, _admin : AdminExtractor, Path(uid): Path<Uuid>) -> Result<impl IntoResponse, ApiError> {
// }
//
//  fn accept_invite(State(state): State<AppState>)-> Result<impl IntoResponse, ApiError>{
// }

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
}
