use reqwest::{Client, IntoUrl, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct Webhook {
	content: String,
	username: String,
	avatar_url: Option<String>,
}
impl Webhook {
	pub fn new(content: &str, username: &str, avatar_url: Option<&str>) -> Self {
		Self {
			content: content.into(),
			username: username.into(),
			avatar_url: avatar_url.map(|x| x.into()),
		}
	}
	pub async fn send<U: IntoUrl>(self, url: U) -> Result<Response, reqwest::Error> {
		Client::new()
			.post(url)
			.header("Content-Type", "application/json")
			.body(serde_json::to_string(&self).unwrap())
			.send()
			.await
			.and_then(|r| r.error_for_status())
	}
}
