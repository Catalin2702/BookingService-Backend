use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::env::Environment;
#[derive(Clone)]
pub struct AppState{
	pub environment: Environment,
	pub database_connection: DatabaseConnection
}

impl AppState {
	pub fn new(
		environment: Environment,
		database_connection: DatabaseConnection
	) -> AppState {
		AppState{
			environment,
			database_connection
		}
	}

	pub async fn create_state() -> AppState {
		let environment = Environment::from_env();
		let config = &environment.database.config;
		let mut options = ConnectOptions::new(environment.database.get_connection_string());
		options
			.max_connections(config.max_connections)
			.min_connections(config.min_connections)
			.connect_timeout(config.connect_timeout)
			.acquire_timeout(config.acquire_timeout)
			.idle_timeout(config.idle_timeout)
			.max_lifetime(config.max_lifetime)
			.sqlx_logging(config.logging)
			.sqlx_logging_level(config.logging_level)
			.set_schema_search_path(environment.database.schema.clone())
			.connect_lazy(true);

		let database_connection = match Database::connect(options).await {
			Ok(value) => value,
			Err(error) => panic!("create_state: Invalid database configuration: {error}")
		};
		AppState::new(environment, database_connection)
	}
}
