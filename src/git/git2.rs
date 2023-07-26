use ::git2::{DiffOptions, Repository};
use anyhow::Result;
use chrono::{FixedOffset, TimeZone};
use pbr::ProgressBar;
use regex::Regex;

use crate::analysis::{Analysis, Change, Commit};
use crate::errors::AnalysisError;
use crate::git::{parse_stat_line, Analyser};

pub struct Git2Analyser {
	repo: Repository,
}

impl Git2Analyser {
	pub fn new() -> Result<Git2Analyser> {
		let repo = match Repository::discover(".") {
			Ok(repo) => repo,
			Err(e) => {
				return Err(AnalysisError::Git {
					message: format!("Can't find Git repository: {:?}", e,),
				}
				.into());
			}
		};

		Ok(Git2Analyser { repo })
	}
}

impl Analyser for Git2Analyser {
	fn get_path(&self) -> Result<std::path::PathBuf> {
		if let Some(path) = self.repo.workdir() {
			Ok(path.to_path_buf())
		} else {
			Err(AnalysisError::Git {
				message: "Repository is bare (has no working directory)".to_owned(),
			}
			.into())
		}
	}

	fn get_commit_count(&self) -> Result<u64> {
		let mut revwalk = self.repo.revwalk()?;
		revwalk.set_sorting(git2::Sort::NONE)?;
		Ok(revwalk.count() as u64)
	}

	fn get_current_revision(&self) -> Result<String> {
		let rev = self.repo.revparse_single("HEAD")?;

		Ok(rev.id().to_string())
	}

	fn analyse(&self, data: &mut Analysis, pb: &mut ProgressBar<std::io::Stdout>) -> Result<()> {
		let mut diffopts = DiffOptions::new();

		let mut revwalk = self.repo.revwalk()?;
		revwalk.push_head()?;

		let word_splitter = Regex::new(r"\b").unwrap();

		for id in revwalk {
			let id = match id {
				Ok(id) => id,
				Err(_) => continue,
			};
			let commit = match self.repo.find_commit(id) {
				Ok(commit) => commit,
				Err(_) => continue,
			};

			if commit.parents().len() > 1 {
				// merge commit
				continue;
			}

			let sig = commit.author();
			let author_name = sig.name().unwrap_or("Unknown").to_string();

			let user_index = data.get_user_index(&author_name);
			data.users[user_index].contributions += 1;

			let git_time = sig.when();
			let time =
				FixedOffset::east(git_time.offset_minutes() * 60).timestamp(git_time.seconds(), 0);

			let a = if commit.parents().len() == 1 {
				let parent = commit.parent(0)?;
				Some(parent.tree()?)
			} else {
				None
			};
			let b = commit.tree()?;

			let diff = self
				.repo
				.diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts))?;

			let words = if let Some(message) = commit.message_raw() {
				data.filter_stop_words(word_splitter.split(message))
			} else {
				vec![]
			};

			let mut c = Commit {
				id: id.to_string(),
				time,
				user: user_index,
				files: vec![],
				words,
			};

			let stats = diff.stats()?;
			let buf = stats.to_buf(git2::DiffStatsFormat::NUMBER, 0)?;
			for line in std::str::from_utf8(&buf)?.lines() {
				let (path, additions, deletions) = parse_stat_line(line, false)?;
				c.files.push(Change {
					file: data.get_file_index(path),
					additions,
					deletions,
				});
			}

			data.commits.push(c);
			pb.inc();
		}

		Ok(())
	}
}
