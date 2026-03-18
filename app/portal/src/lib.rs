#[path = "utils/tag_format.rs"]
mod tag_format;

use html_codec::HTMLEncode as _;
use wasm_bindgen::prelude::*;

use self::tag_format::CommonTag;

#[wasm_bindgen]
pub fn format_tag(text: &str, quot: bool) -> String {
	text.br().escape(quot).tag(CommonTag).into()
}

#[wasm_bindgen]
pub fn format_tag_and_link(text: &str) -> String {
	text.br().escape_and_link().tag(CommonTag).into()
}
