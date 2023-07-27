use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalysisError {
	#[error("Git client error ({0})", message)]
	Git { message: String },
	#[error("Parse error ({0})", message)]
	Parse { message: &'static str },
	#[error("Upload error ({0})", message)]
	Upload { message: String },
}

#[derive(Error, Debug)]
pub enum UserError {
	#[error("Invalid target directory")]
	InvalidTarget { message: String },
	#[error("Sorry, we're not able to support repos with this many commits at the moment. We're working on it!")]
	TooBig {},
}
