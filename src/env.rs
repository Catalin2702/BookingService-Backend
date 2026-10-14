use std::net::SocketAddr;

fn read_env_var(name: &str) -> String {
	use std::env;
	match env::var(name) {
		Ok(value) => value,
		Err(error) => panic!("read_env_var: The {name} variable raise an error: {error}")
	}
}

fn read_secret(name: &str) -> String {
	let file_name = format!("{name}_FILE");
	match std::env::var(&file_name) {
		Ok(path) => match std::fs::read_to_string(&path) {
			Ok(value) => value.trim_end().to_string(),
			Err(error) => panic!("read_secret: Cannot read {file_name} ({path}): {error}")
		},
		Err(_) => read_env_var(name)
	}
}

#[derive(Debug)]
pub struct BackendEnvironment {
	pub host: String,
	pub port: String
}

impl BackendEnvironment {
	fn from_env() -> BackendEnvironment {
		BackendEnvironment{
			host: read_env_var("BACKEND_HOST"),
			port: read_env_var("BACKEND_PORT"),
		}
	}
}

#[derive(Debug)]
pub struct DatabaseEnvironment {
	pub host: String,
	pub port: String,
	pub database: String,
	pub user: String,
	pub password: String
}

impl DatabaseEnvironment {
	fn from_env() -> DatabaseEnvironment {
		DatabaseEnvironment{
			host: read_env_var("DATABASE_HOST"),
			port: read_env_var("DATABASE_PORT"),
			database: read_env_var("DATABASE_DB"),
			user: read_env_var("DATABASE_USER"),
			password: read_secret("DATABASE_PASSWORD"),
		}
	}
}

#[derive(Debug)]
pub struct Environment {
	pub backend: BackendEnvironment,
	pub database: DatabaseEnvironment
}

impl Environment {
	pub fn from_env() -> Environment {
		Environment{
			backend: BackendEnvironment::from_env(),
			database: DatabaseEnvironment::from_env()
		}
	}
}

pub fn get_backend_address() -> SocketAddr {
	let environment = Environment::from_env();
	let url = format!("{}:{}", environment.backend.host, environment.backend.port).to_string();

	url.parse::<SocketAddr>().unwrap()
}
