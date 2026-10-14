use axum::Router;
use axum::routing::get;

async fn get_health() -> &'static str {
	"ok"
}

pub fn get_router_health() -> Router {
	Router::new()
		.route("/health", get(get_health))
}
