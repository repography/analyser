use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

use crate::errors::AnalysisError;

#[cfg(feature = "tls")]
use attohttpc_tls as attohttpc;

const API_BASE: &str = "https://repography.com";

pub struct Api {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateResponse {
	pub upload_url: String,
	pub project_id: String,
	pub analysis_key: String,
}

impl Api {
	pub fn create(&self, name: String, analysis_hash: &String) -> Result<CreateResponse> {
		let url = format!("{}/api/project/create", API_BASE);

		let mut params = HashMap::new();
		params.insert("name", &name);
		params.insert("analysis_hash", analysis_hash);
		let body = serde_json::to_vec(&params)?;

		let res = attohttpc::post(url).bytes(&body).send()?;

		let bytes = res.bytes()?;
		Ok(serde_json::from_reader(&*bytes)?)
	}

	pub fn progress(&self, progress_id: &str, percent: u8, message: &str) -> Result<String> {
		let url = format!(
			"{}/api/project/progress/{}/{}",
			API_BASE, progress_id, percent
		);
		let res = attohttpc::post(url).text(message).send()?;

		Ok(res.text()?)
	}

	pub fn upload(&self, upload_url: String, data: &Vec<u8>) -> Result<()> {
		let res = attohttpc::put(upload_url)
			.header(attohttpc::header::CONTENT_TYPE, "application/octet-stream")
			.bytes(data)
			.send()?;

		if res.is_success() {
			Ok(())
		} else {
			Err(AnalysisError::Upload {
				message: format!("Response: {:?}", res.text()?,),
			}
			.into())
		}
	}

	pub fn render(&self, project_id: &String, progress_id: Option<&str>) -> Result<String> {
		let url = format!(
			"{}/api/project/render/{}/{}",
			API_BASE,
			project_id,
			progress_id.unwrap_or("null"),
		);

		let res = attohttpc::post(url)
			.header(attohttpc::header::CONTENT_LENGTH, 0)
			.read_timeout(Duration::new(60, 0))
			.send()?;

		Ok(res.text()?)
	}
}
