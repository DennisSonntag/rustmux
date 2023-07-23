use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
	name: SessionName,
	root_dir: RootDir,
	windows: Windows,
}

#[derive(Debug, Deserialize)]
enum SessionName {
	#[serde(rename = "arg")]
	Arg,
	#[serde(untagged)]
	Name(String),
}

#[derive(Debug, Deserialize)]
enum RootDir {
	#[serde(rename = "arg")]
	Arg,
	#[serde(untagged)]
	Path(PathBuf),
}

#[derive(Debug, Deserialize)]
struct Windows {
	#[serde(flatten)]
	commands: HashMap<String, Command>,
}

#[derive(Debug, Deserialize)]
struct Command {
	cmd: Commands,
}

#[derive(Debug, Deserialize)]
enum Commands {
	#[serde(untagged)]
	SingleCommand(String),
	#[serde(untagged)]
	MultipleCommands(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct Pane {
	split: Option<Split>,
	size: usize,
}

#[derive(Debug, Deserialize)]
enum Split {
	Vertical,
	Horizontal,
}
