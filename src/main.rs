mod routers;
mod env;

use tokio::net::TcpListener;

use crate::{
	env::get_backend_address,
	routers::get_router
};

#[tokio::main]
async fn main() {
	let health_router = get_router();

	let address = get_backend_address();
	let listener = TcpListener::bind(address).await
		.unwrap_or_else(|error| panic!("main: Can't bind to {address}: {error}"));

	axum::serve(listener, health_router).await.unwrap()
}
