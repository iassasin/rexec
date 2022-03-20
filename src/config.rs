use std::{collections::HashMap, fs};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
	pub http: HttpConfig,
	pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
	pub listen_ip: String,
	pub port: u16,
	pub max_threads: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
	pub command: String,
}

impl AppConfig {
	pub fn read_from_file<P: AsRef<str>>(path: P) -> AppConfig {
		serde_yaml::from_str(
			fs::read_to_string(path.as_ref()).expect("Unable to read rexec.yml").as_str()
		).expect("Unable to parse rexec.yml")
	}
}