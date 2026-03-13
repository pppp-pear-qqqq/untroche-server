use serde::Serialize;

use crate::utils::resource;

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
