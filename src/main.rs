#![allow(unused, dead_code)]
use crate::serialize::Config;

extern crate tmux_interface;

extern crate dirs;

use tmux_interface::{HasSession, KillSession, NewSession, NewWindow, PaneSize, SplitWindow, Tmux};

use std::{env, fs::read_to_string, path::PathBuf};

use serde_json::from_str;

use anyhow::Result;

use walkdir::WalkDir;



mod serialize;

struct ConfigBuilder {
	path: PathBuf,
	args: Vec<String>,
	is_tmux: bool,
}

impl ConfigBuilder {
	fn new() -> Self {
		let args: Vec<String> = env::args().collect();

		let mut path = PathBuf::new();
		path.push(dirs::home_dir().expect("could not find home dir"));
		path.push(dirs::config_dir().expect("could not find config dir"));
		path.push("rustmux");

		let is_tmux = matches!(env::var("TMUX"), Ok(_));

		Self {
			path,
			args,
			is_tmux,
		}
	}
	fn valid_config(&self) -> bool {
		WalkDir::new(self.path.clone())
			.into_iter()
			.map(|entry| {
				let entry = entry.unwrap();
				return if entry.file_type().is_dir() {
					None
				} else {
					Some(entry.file_name().to_string_lossy().replace(".json", ""))
				};
			})
			.filter(Option::is_some)
			.flatten()
			.any(|x| x == self.args[1])
	}
	fn get_config(mut self) -> Result<Config> {
		self.path.push(format!("{}.json", self.args[1]));
		let json = read_to_string(self.path)?;

		let config = from_str::<Config>(&json)?;

		Ok(config)
	}
}

fn main() -> Result<()> {
	let config_builder = ConfigBuilder::new();

	if config_builder.args.len() > 1 {
		if config_builder.valid_config() {
			let config = config_builder.get_config()?;
			println!("{config:#?}");
		} else {
			println!("config does not exist");
		}
	} else {
		println!("please provide a name");
	}

	Ok(())
}

// fn main() {
// 	let target_session = "example_1";
//
// 	// tmux new -d -s example_1 ; neww ; splitw -v
// 	Tmux::new()
// 		// .add_command(NewSession::new().detached().session_name(target_session))
// 		.add_command(NewSession::new().attach().session_name(target_session))
// 		.add_command(NewWindow::new())
// 		.add_command(
// 			SplitWindow::new()
// 				.vertical()
// 				.size(&PaneSize::Percentage(50)),
// 		)
// 		.output()
// 		.unwrap();
//
// 	// tmux has -t example_1
// 	let status = Tmux::with_command(HasSession::new().target_session(target_session))
// 		.status()
// 		.unwrap()
// 		.success();
//
// 	// Tmux::with_command(command)
// 	//
//
// 	assert!(status);
//
// 	// tmux kill-session -t example_1
// 	Tmux::with_command(KillSession::new().target_session(target_session))
// 		.output()
// 		.unwrap();
// }
