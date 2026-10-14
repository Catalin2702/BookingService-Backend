use axum::Router;
use crate::state::AppState;

mod health;

pub fn get_router(state: AppState) -> Router {
	Router::new()
		.merge(health::get_router_health())
		.with_state(state)
}
