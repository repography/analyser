mod analysis;
mod api;
mod encode;
mod errors;
mod git;
mod languages;

use anyhow::Result;
use clap::Parser;
use dialoguer::Input;
use pbr::ProgressBar;
use std::collections::HashMap;
use std::path::Path;

use crate::analysis::{Analysis, Language};
use crate::api::Api;
use crate::encode::encode;
use crate::errors::{AnalysisError, UserError};
use crate::git::{get_analyser as get_git_analyser, Analyser as GitAnalyser};
use crate::languages::{get_language_index, LANG_NAMES};

const NUM_STEPS: u8 = 5;
const MAX_COMMITS: u64 = 100_000;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
	/// Progress ID to update the web frontend.
	progress_id: Option<String>,

	/// Path to the Git repo to analyse.
	#[arg(short, long)]
	target: Option<String>,
}

fn get_name(git: &dyn GitAnalyser) -> Result<String> {
	let repo_path = git.get_path()?;
	let default_name = match repo_path.file_name() {
		Some(name) => name.to_string_lossy(),
		None => {
			return Err(AnalysisError::Parse {
				message: "Missing repo path",
			}
			.into());
		}
	};

	let prompt = "[2/5]\tPlease enter a name for this project";

	let response: String = Input::new()
		.with_prompt(prompt)
		.default(default_name.to_string())
		.interact()?;

	if response.is_empty() {
		Ok(default_name.to_string())
	} else {
		Ok(response)
	}
}

fn progress<S: AsRef<str>>(
	api: &Api,
	progress_id: Option<&str>,
	step: u8,
	percent: u8,
	message: S,
) -> Result<()> {
	println!("[{}/{}]\t{}", step, NUM_STEPS, message.as_ref());

	if let Some(progress_id) = progress_id {
		if percent == 100 {
			// The render call sends the final progress notification with the public URL,
			// we don't need to notify the API ourselves.
		} else {
			api.progress(progress_id, percent, message.as_ref())?;
		}
	}

	Ok(())
}

fn run() -> Result<()> {
	let args = Args::parse();
	let progress_id = args.progress_id.as_deref();

	// Remember the original working dir so we can restore it.
	let original_working_dir = std::env::current_dir()?;

	if let Some(target) = args.target.as_deref() {
		if let Err(e) = std::env::set_current_dir(target) {
			return Err(UserError::InvalidTarget {
				message: e.to_string(),
			}
			.into());
		}
	}

	// Change dir to the root of the repo.
	let git = get_git_analyser()?;
	std::env::set_current_dir(git.get_path()?)?;

	let api = Api {};
	progress(&api, progress_id, 1, 20, "Reading commits...")?;

	// Read the total number of commits.
	let num_commits = git.get_commit_count()?;
	if num_commits > MAX_COMMITS {
		return Err(UserError::TooBig {}.into());
	}

	// Read each commit in the log.
	let mut pb = ProgressBar::new(num_commits);
	let rev = git.get_current_revision()?;
	let mut analysis = Analysis::new(&rev);
	git.analyse(&mut analysis, &mut pb)?;
	pb.finish();

	// Get the size of each file for the language stats.
	let language_index = get_language_index();
	let mut languages: HashMap<usize, usize> = HashMap::new();
	let mut total_bytes: u64 = 0;
	for path_str in &analysis.files {
		let path = Path::new(&path_str);
		if let Ok(metadata) = std::fs::metadata(path) {
			let len = metadata.len();
			total_bytes += len;

			if let Some(ext) = path.extension() {
				let ext_str = ext.to_string_lossy().into_owned();
				if let Some(i) = language_index.get::<str>(&ext_str) {
					let count = languages.entry(*i).or_insert(0);
					*count += len as usize;
				}
			}
		}
	}

	analysis.total_bytes = total_bytes as usize;
	for (language_index, bytes) in languages {
		analysis.languages.push(Language {
			name: LANG_NAMES[language_index].to_string(),
			bytes,
		});
	}

	// Create repo on Repography.
	let name = get_name(&*git)?;
	let res = api.create(name, &rev)?;

	// Upload analysis.
	progress(&api, progress_id, 3, 80, "Uploading analysis...")?;
	let encoded = encode(&analysis, &res.analysis_key)?;
	api.upload(res.upload_url, &encoded)?;

	// Trigger render.
	progress(&api, progress_id, 4, 90, "Rendering...")?;
	let public_url = api.render(&res.project_id, progress_id)?;

	// Wrap up.
	progress(
		&api,
		progress_id,
		5,
		100,
		format!(
			"All done - you can see the results for your repo at: {}",
			&public_url
		),
	)?;
	std::env::set_current_dir(original_working_dir)?;
	Ok(())
}

fn main() {
	let mut log_builder = env_logger::Builder::from_default_env();
	log_builder.target(env_logger::Target::Stderr);
	log_builder.init();

	if let Err(e) = run() {
		if let Some(ue) = e.downcast_ref::<UserError>() {
			eprintln!("Error: {}", ue);
			return;
		}

		eprintln!("Sorry something has gone wrong. This might be an problem on our side so please try again later.");
		eprintln!("If this continues, please let us know at support@repography.com.\n");
		eprintln!(
			"Technical details which might help us understand the problem:\n\t{}",
			e
		);
	}
}
