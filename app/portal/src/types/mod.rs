pub mod app_data;
pub mod error;
pub mod state;
pub mod tag_format;

use serde::Deserialize;

pub use self::{app_data::AppData, error::*, state::State, tag_format::CommonTag};

pub type StateHandle = common::StateHandle<State>;
pub type Id = common::Identity<String>;

#[derive(Deserialize)]
pub struct PageParams {
	#[serde(default = "usize::default")]
	pub page: usize,
	#[serde(default = "page_limit_default")]
	pub limit: usize,
}
impl PageParams {
	pub fn offset(&self) -> usize {
		self.page * self.limit
	}
}

fn page_limit_default() -> usize {
	20
}
