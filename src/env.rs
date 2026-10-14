use log::LevelFilter;
use std::{
	env,
	fmt::Display,
	fs::read_to_string,
	net::SocketAddr,
	str::FromStr,
	time::Duration
};

fn read_env_var(name: &str) -> String {
	match env::var(name) {
		Ok(value) => value,
		Err(error) => panic!("read_env_var: The {name} variable raise an error: {error}")
	}
}

fn read_env_var_default<T: FromStr>(name: &str, default: T) -> T where <T as FromStr>::Err: Display {
	match env::var(name) {
		Ok(value) => match value.parse::<T>() {
			Ok(parsed_value) => parsed_value,
			Err(error) => panic!("read_env_var_default: The {name} variable raise an error: {error}")
		},
		Err(_) => default
	}
}

fn read_env_secs_default(name: &str, default_secs: u64) -> Duration {
	Duration::from_secs(read_env_var_default(name, default_secs))
}

fn read_secret(name: &str) -> String {
	let file_name = format!("{name}_FILE");
	match env::var(&file_name) {
		Ok(path) => match read_to_string(&path) {
			Ok(value) => value.trim_end().to_string(),
			Err(error) => panic!("read_secret: Cannot read {file_name} ({path}): {error}")
		},
		Err(_) => read_env_var(name)
	}
}

mod vars {
	pub mod backend {
		pub const HOST: &str = "BACKEND_HOST";
		pub const PORT: &str = "BACKEND_PORT";
	}

	pub mod database {
		pub mod config {
			pub const MAX_CONNECTIONS: &str = "DATABASE_MAX_CONNECTIONS";
			pub const MIN_CONNECTIONS: &str = "DATABASE_MIN_CONNECTIONS";
			pub const CONNECT_TIMEOUT: &str = "DATABASE_CONNECT_TIMEOUT";
			pub const ACQUIRE_TIMEOUT: &str = "DATABASE_ACQUIRE_TIMEOUT";
			pub const IDLE_TIMEOUT: &str = "DATABASE_IDLE_TIMEOUT";
			pub const MAX_LIFETIME: &str = "DATABASE_MAX_LIFETIME";
			pub const TEST_BEFORE_ACQUIRE: &str = "DATABASE_TEST_BEFORE_ACQUIRE";
			pub const LOGGING: &str = "DATABASE_LOGGING";
			pub const LOGGING_LEVEL: &str = "DATABASE_LOGGING_LEVEL";
		}
		pub const HOST: &str = "DATABASE_HOST";
		pub const PORT: &str = "DATABASE_PORT";
		pub const DB: &str = "DATABASE_DB";
		pub const SCHEMA: &str = "DATABASE_SCHEMA";
		pub const PROTOCOL: &str = "DATABASE_PROTOCOL";
		pub const USER: &str = "DATABASE_USER";
		pub const PASSWORD: &str = "DATABASE_PASSWORD";
	}
}

#[derive(Clone, Debug)]
pub struct BackendEnvironment {
	pub host: String,
	pub port: String,
}

impl BackendEnvironment {
	pub fn new(
		host: String,
		port: String,
	) -> BackendEnvironment {
		BackendEnvironment{
			host,
			port,
		}
	}

	pub fn from_env() -> BackendEnvironment {
		BackendEnvironment::new(
			read_env_var(vars::backend::HOST),
			read_env_var(vars::backend::PORT),
		)
	}

	pub fn get_address(&self) -> SocketAddr {
		match format!("{}:{}", self.host, self.port).to_string().parse::<SocketAddr>() {
			Ok(value) => value,
			Err(error) => panic!("BackendEnvironment::get_address: Can't get the address: {error}")
		}
	}
}

#[derive(Clone, Debug)]
pub struct DatabaseConfigEnvironment {
	pub max_connections: u32,
	pub min_connections: u32,

	pub connect_timeout: Duration,
	pub acquire_timeout: Duration,
	pub idle_timeout: Duration,
	pub max_lifetime: Duration,

	pub test_before_acquire: bool,

	pub logging: bool,
	pub logging_level: LevelFilter,
}

impl DatabaseConfigEnvironment {
	pub fn new(
		max_connections: u32,
		min_connections: u32,
		connect_timeout: Duration,
		acquire_timeout: Duration,
		idle_timeout: Duration,
		max_lifetime: Duration,
		test_before_acquire: bool,
		logging: bool,
		logging_level: LevelFilter,
	) -> DatabaseConfigEnvironment {
		DatabaseConfigEnvironment{
			max_connections,
			min_connections,
			connect_timeout,
			acquire_timeout,
			idle_timeout,
			max_lifetime,
			test_before_acquire,
			logging,
			logging_level,
		}
	}

	pub fn from_env() -> DatabaseConfigEnvironment {
		use vars::database::config;
		DatabaseConfigEnvironment::new(
			read_env_var_default(config::MAX_CONNECTIONS, 10),
			read_env_var_default(config::MIN_CONNECTIONS, 0),
			read_env_secs_default(config::CONNECT_TIMEOUT, 10),
			read_env_secs_default(config::ACQUIRE_TIMEOUT, 30),
			read_env_secs_default(config::IDLE_TIMEOUT, 10 * 60),
			read_env_secs_default(config::MAX_LIFETIME, 30 * 60),
			read_env_var_default(config::TEST_BEFORE_ACQUIRE, true),
			read_env_var_default(config::LOGGING, false),
			read_env_var_default(config::LOGGING_LEVEL, LevelFilter::Info),
		)
	}
}

#[derive(Clone, Debug)]
pub struct DatabaseEnvironment {
	pub host: String,
	pub port: String,

	pub database: String,
	pub schema: String,
	pub protocol: String,

	pub user: String,
	pub password: String,

	pub config: DatabaseConfigEnvironment,
}

impl DatabaseEnvironment {
	pub fn new(
		host: String,
		port: String,
		database: String,
		schema: String,
		protocol: String,
		user: String,
		password: String,
		config: DatabaseConfigEnvironment,
	) -> DatabaseEnvironment {
		DatabaseEnvironment{
			host,
			port,
			database,
			schema,
			protocol,
			user,
			password,
			config,
		}
	}

	pub fn from_env() -> DatabaseEnvironment {
		DatabaseEnvironment::new(
			read_env_var(vars::database::HOST),
			read_env_var(vars::database::PORT),
			read_env_var(vars::database::DB),
			read_env_var(vars::database::SCHEMA),
			read_env_var(vars::database::PROTOCOL),
			read_env_var(vars::database::USER),
			read_secret(vars::database::PASSWORD),
			DatabaseConfigEnvironment::from_env(),
		)
	}

	pub fn get_connection_string(&self) -> String {
		format!(
			"{protocol}://{user}:{password}@{host}:{port}/{database}",
			protocol=self.protocol,
			user=self.user,
			password=self.password,
			host=self.host,
			port=self.port,
			database=self.database
		).to_string()
	}
}

#[derive(Clone, Debug)]
pub struct Environment {
	pub backend: BackendEnvironment,
	pub database: DatabaseEnvironment,
}

impl Environment {
	pub fn new(
		backend_environment: BackendEnvironment,
		database_environment: DatabaseEnvironment,
	) -> Environment {
		Environment{
			backend: backend_environment,
			database: database_environment,
		}
	}

	pub fn from_env() -> Environment {
		Environment::new(
			BackendEnvironment::from_env(),
			DatabaseEnvironment::from_env(),
		)
	}
}
