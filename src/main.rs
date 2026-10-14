use tokio::net::TcpListener;

use booking_service::{
	routers::get_router,
	state::AppState
};

#[tokio::main]
async fn main() {
	let state = AppState::create_state().await;
	let address = state.environment.backend.get_address();

	let health_router = get_router(state);
	let listener = TcpListener::bind(address).await
		.unwrap_or_else(|error| panic!("main: Can't bind to {address}: {error}"));

	axum::serve(listener, health_router).await.unwrap()
}
