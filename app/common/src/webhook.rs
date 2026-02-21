use reqwest::{Client, IntoUrl, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct Webhook {
	content: String,
	username: String,
	avatar_url: Option<String>,
}
impl Webhook {
	pub fn new<T0: AsRef<str>, T1: AsRef<str>, T2: AsRef<str>>(content: T0, username: T1, avatar_url: Option<T2>) -> Self {
		Self {
			content: content.as_ref().to_string(),
			username: username.as_ref().to_string(),
			avatar_url: avatar_url.map(|x| x.as_ref().to_string()),
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
