use serde::{Deserialize as _, Deserializer, Serialize};

pub mod password {
	use argon2::{
		Argon2, PasswordHasher as _, PasswordVerifier as _,
		password_hash::{Error, Result, SaltString, rand_core::OsRng},
	};

	pub fn hash(password: &str) -> Result<String> {
		Ok(Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))?.to_string())
	}

	pub fn verify(password: &str, hashed: &str) -> Result<bool> {
		match Argon2::default().verify_password(password.as_bytes(), &argon2::PasswordHash::new(&hashed)?) {
			Ok(_) => Ok(true),
			Err(Error::Password) => Ok(false),
			Err(err) => Err(err),
		}
	}
}

/// テンプレート種別
pub enum Template {
	Base { summary: Option<Summary> },
	Popup,
	None,
}
#[derive(Serialize)]
pub struct Summary {
	pub name: String,
}
impl Template {
	/// テンプレートからHTMLを生成
	pub fn render(self, file: &str, globals: impl liquid::ObjectView) -> Result<String, liquid::Error> {
		let parser = liquid::ParserBuilder::with_stdlib().build()?;
		let main = parser.parse_file(resource(file))?.render(&globals)?;
		match self {
			Template::Base { summary } => parser.parse_file(resource("template/base.html"))?.render(&liquid::object!({
				"main": &main,
				"summary": &summary,
			})),
			Template::Popup => parser.parse_file(resource("template/popup.html"))?.render(&liquid::object!({
				"main": &main,
			})),
			Template::None => Ok(main),
		}
	}
	pub fn render_raw(self, text: &str) -> Result<String, liquid::Error> {
		let parser = liquid::ParserBuilder::with_stdlib().build()?;
		match self {
			Template::Base { summary } => parser.parse_file(resource("template/base.html"))?.render(&liquid::object!({
				"main": &text,
				"summary": &summary,
			})),
			Template::Popup => parser.parse_file(resource("template/popup.html"))?.render(&liquid::object!({
				"main": &text,
			})),
			Template::None => Ok(text.into()),
		}
	}
}

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
