use anyhow::Result;
use pbr::ProgressBar;

#[cfg(not(feature = "git-libgit2"))]
mod cli;
#[cfg(feature = "git-libgit2")]
mod git2;

use crate::analysis::Analysis;
use crate::errors::AnalysisError;
#[cfg(not(feature = "git-libgit2"))]
use crate::git::cli::CliAnalyser as ConcreteAnalyser;
#[cfg(feature = "git-libgit2")]
use crate::git::git2::Git2Analyser as ConcreteAnalyser;

pub trait Analyser {
	fn get_path(&self) -> Result<std::path::PathBuf>;
	fn get_commit_count(&self) -> Result<u64>;
	fn get_current_revision(&self) -> Result<String>;
	fn analyse(&self, data: &mut Analysis, pb: &mut ProgressBar<std::io::Stdout>) -> Result<()>;
}

pub fn get_analyser() -> Result<Box<dyn Analyser>> {
	let analyser = ConcreteAnalyser::new()?;
	Ok(Box::new(analyser))
}

pub fn parse_num_changes(s: &str) -> usize {
	let trimmed = s.trim_end();
	if trimmed == "-" {
		return 0;
	}
	trimmed.parse::<usize>().unwrap_or(0)
}

pub fn parse_stat_line(line: &str, hard_tabs: bool) -> Result<(&str, usize, usize)> {
	let (path, additions_str, deletions_str) = if hard_tabs {
		let parts: Vec<&str> = line.splitn(3, '\t').collect();
		if parts.len() != 3 {
			return Err(AnalysisError::Git {
				message: format!("Malformed stat line: {:?}", line),
			}
			.into());
		}

		(parts[2], parts[0], parts[1])
	} else {
		if line.len() < 16 {
			return Err(AnalysisError::Git {
				message: format!("Malformed stat line: {:?}", line),
			}
			.into());
		}

		let (additions_str, rest) = line.split_at(8);
		let (deletions_str, path) = rest.split_at(8);
		(path, additions_str, deletions_str)
	};

	Ok((
		path,
		parse_num_changes(additions_str),
		parse_num_changes(deletions_str),
	))
}
