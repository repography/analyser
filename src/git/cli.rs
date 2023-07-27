use anyhow::Result;
use chrono::DateTime;
use log::warn;
use pbr::ProgressBar;
use std::io::{BufRead, BufReader};
use subprocess::{Exec, Redirection};

use crate::analysis::{Analysis, Change, Commit};
use crate::errors::AnalysisError;
use crate::git::{parse_stat_line, Analyser};

fn call_git(args: Vec<&'static str>) -> Result<String> {
	let mut command = Exec::cmd("git");
	for arg in &args {
		command = command.arg(arg);
	}

	let capture = command
		.stdout(Redirection::Pipe)
		.stderr(Redirection::Pipe)
		.capture()?;

	if capture.success() {
		Ok(capture.stdout_str().trim().to_string())
	} else {
		Err(AnalysisError::Git {
			message: capture.stderr_str(),
		}
		.into())
	}
}

fn call_git_stream(args: Vec<&'static str>) -> Result<Box<dyn std::io::Read>> {
	let mut command = Exec::cmd("git");
	for arg in &args {
		command = command.arg(arg);
	}

	Ok(Box::new(command.stream_stdout()?))
}

struct LossyLineReader {
	br: BufReader<Box<dyn std::io::Read>>,
}

impl LossyLineReader {
	pub fn new(stream: Box<dyn std::io::Read>) -> Self {
		LossyLineReader {
			br: BufReader::new(stream),
		}
	}
}

impl Iterator for LossyLineReader {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		let mut buf = vec![];
		if let Ok(len) = self.br.read_until(b'\n', &mut buf) {
			return match len {
				0 => None,
				1 => Some("".to_string()),
				_ => match std::str::from_utf8(&buf[0..len - 1]) {
					Ok(line) => Some(line.to_string()),
					Err(e) => {
						warn!("Invalid UTF-8: {:?}", e);
						Some("".to_string())
					}
				},
			};
		}

		None
	}
}

pub struct CliAnalyser {}

impl CliAnalyser {
	pub fn new() -> Result<CliAnalyser> {
		call_git(vec!["--version"])?;
		Ok(CliAnalyser {})
	}
}

impl Analyser for CliAnalyser {
	fn get_path(&self) -> Result<std::path::PathBuf> {
		let out = call_git(vec!["rev-parse", "--show-toplevel"])?;
		Ok(std::path::PathBuf::from(out.trim_end()))
	}

	fn get_commit_count(&self) -> Result<u64> {
		let count_str = call_git(vec!["rev-list", "--count", "HEAD"])?;

		Ok(count_str.parse::<u64>()?)
	}

	fn get_current_revision(&self) -> Result<String> {
		let rev = call_git(vec!["rev-parse", "HEAD"])?;

		Ok(rev)
	}

	fn analyse(&self, data: &mut Analysis, pb: &mut ProgressBar<std::io::Stdout>) -> Result<()> {
		let log_stream = call_git_stream(vec![
			"log",
			"--all",
			"--numstat",
			"--date=rfc2822",
			"--use-mailmap",
			"--pretty=format:#%H#%ad#%aN#%f",
			"--no-renames",
			"--no-expand-tabs",
			"--no-merges",
			"--encoding=UTF-8",
		])?;

		let log_reader = LossyLineReader::new(log_stream);

		let mut commit = None;
		for line in log_reader {
			if line.len() < 2 {
				if let Some(commit) = commit.take() {
					data.commits.push(commit);
				}
				continue;
			}

			if let Some(marker) = line.strip_prefix('#') {
				pb.inc();

				let mut parts = marker.splitn(4, '#');

				let id = parts.next().ok_or(AnalysisError::Parse {
					message: "Missing commit hash",
				})?;

				let time_str = parts.next().ok_or(AnalysisError::Parse {
					message: "Missing time",
				})?;

				let time = DateTime::parse_from_rfc2822(time_str)?;

				let author_name = parts.next().unwrap_or("Unknown");

				let words = match parts.next() {
					Some(words) => data.filter_stop_words(words.split('-')),
					None => vec![],
				};

				let user_index = data.get_user_index(author_name);
				data.users[user_index].contributions += 1;

				commit = Some(Commit {
					id: id.to_string(),
					time,
					user: user_index,
					files: vec![],
					words,
				});
			} else if let Some(ref mut commit) = commit {
				let (path, additions, deletions) = parse_stat_line(&line, true)?;
				commit.files.push(Change {
					file: data.get_file_index(path),
					additions,
					deletions,
				});
			}
		}

		if let Some(commit) = commit.take() {
			data.commits.push(commit);
		}

		Ok(())
	}
}
