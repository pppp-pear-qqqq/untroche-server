use serde::Serialize;

use super::resource;

/// テンプレート種別
pub enum Template {
	Base { nobots: bool, summary: Option<Summary>, user: Option<User> },
	None,
}
#[derive(Serialize)]
pub struct Summary {
	pub title: String,
	pub desc: String,
	pub url: String,
	pub ogtype: String,
	pub image: String,
	pub card: String,
}
#[derive(Serialize)]
pub struct User {
	pub name: String,
}
impl Template {
	/// テンプレートからHTMLを生成
	pub fn render(self, file: &str, globals: impl liquid::ObjectView) -> Result<String, liquid::Error> {
		let parser = liquid::ParserBuilder::with_stdlib().build()?;
		let main = parser.parse_file(resource(file))?.render(&globals)?;
		match self {
			Template::Base { nobots, summary, user } => parser.parse_file(resource("template.html"))?.render(&liquid::object!({
				"main": &main,
				"nobots": &nobots,
				"summary": &summary,
				"user": &user,
			})),
			Template::None => Ok(main),
		}
	}
	pub fn render_raw(self, text: &str) -> Result<String, liquid::Error> {
		let parser = liquid::ParserBuilder::with_stdlib().build()?;
		match self {
			Template::Base { nobots, summary, user } => parser.parse_file(resource("template.html"))?.render(&liquid::object!({
				"main": &text,
				"nobots": &nobots,
				"summary": &summary,
				"user": &user,
			})),
			Template::None => Ok(text.into()),
		}
	}
}
