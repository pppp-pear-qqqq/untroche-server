pub mod app_data;
pub mod error;
pub mod page_params;
pub mod state;
pub mod tag_format;
pub mod template;

use serde::{Deserialize as _, Deserializer};

pub use self::{app_data::AppData, error::*, page_params::PageParams, state::State, tag_format::CommonTag, template::Template};

pub type StateHandle = common::StateHandle<State>;
pub type Eno = common::Identity<i64>;

// 変数定義
pub const STATE: &str = "STATE";
const KEY: &str = "KEY";

/// リソースへのパスを生成する
pub fn resource(path: &str) -> String {
	if cfg!(debug_assertions) {
		format!("{}/resource/{path}", crate::APP_PATH)
	} else {
		format!("{}/{path}", crate::APP_PATH)
	}
}

/// クエリやフォームからboolを取得
pub fn deser_flag<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	match s.as_str() {
		"" | "true" | "1" => Ok(true),
		"false" | "0" => Ok(false),
		_ => Err(serde::de::Error::custom("boolean flag expected")),
	}
}
