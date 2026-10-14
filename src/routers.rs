use axum::Router;

mod health;

pub fn get_router() -> Router {
	Router::new()
		.merge(health::get_router_health())
}
