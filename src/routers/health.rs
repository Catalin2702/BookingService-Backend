use axum::{
	Router,
	routing::get,
	extract::State,
	http::StatusCode
};
use crate::state::AppState;

async fn get_health() -> (StatusCode, &'static str) {
	(StatusCode::OK, "ok")
}

async fn get_db_health(State(state): State<AppState>) -> (StatusCode, &'static str) {
	match state.database_connection.ping().await {
		Ok(_) => (StatusCode::OK, "ok"),
		Err(error) => {
			eprintln!("get_db_health: Database unreachable: {error}");
			(StatusCode::SERVICE_UNAVAILABLE, "database unavailable")
		}
	}
}

pub fn get_router_health() -> Router<AppState> {
	Router::<AppState>::new()
		.route("/health", get(get_health))
		.route("/db-health", get(get_db_health))
}
