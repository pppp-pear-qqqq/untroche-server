use std::borrow::Cow;

use html_codec::TagFormat;
use rand::seq::IndexedRandom as _;

#[derive(Clone, Copy)]
pub struct CommonTag;

impl TagFormat for CommonTag {
	fn parse(self, raw: &str) -> Cow<'_, str> {
		fn part(value: &str, limit: usize) -> Vec<&str> {
			let mut parts = Vec::new();
			let mut nest: usize = 0;
			let mut bytes = value.bytes().enumerate().peekable();
			while let Some((idx, b)) = bytes.next() {
				match b {
					b'[' => nest += 1,
					b']' if nest > 0 => nest -= 1,
					b'|' if nest == 0 => {
						parts.push(idx);
						if limit != 0 && limit <= parts.len() {
							break;
						}
					}
					b'\\' => {
						bytes.next_if(|(_, b)| matches!(b, b'[' | b']' | b'|' | b'\\'));
					}
					_ => (),
				}
			}
			let mut start = 0;
			let mut params = Vec::with_capacity(parts.len() + 1);
			for end in parts {
				params.push(&value[start..end]);
				start = end + 1;
			}
			params.push(&value[start..]);
			params
		}
		let mut out = String::with_capacity(raw.len() * 2);
		let mut rng = rand::rng();
		let mut end = 0;
		let mut stack = Vec::with_capacity(1);
		let mut bytes = raw.bytes().enumerate().peekable();
		while let Some((idx, b)) = bytes.next() {
			match b {
				b'[' => {
					let start = idx + 1;
					// ネストが無い時にはそれまでを出力
					if stack.is_empty() {
						out.push_str(&raw[end..idx]);
						end = start;
					}
					// タグ名取得
					if let Some(p) = raw[start..].find('/') {
						let tag = &raw[start..start + p];
						// タグかどうか
						if tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
							stack.push((tag, start + p + 1));
							continue;
						}
					}
					// 無名タグ
					stack.push(("", start));
				}
				b']' if !stack.is_empty() => {
					// タグ名取得
					let (p, tag) = raw[end..idx]
						.rfind('/')
						.and_then(|p| {
							let tag = &raw[end + p + 1..idx];
							tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_').then_some((end + p, tag))
						})
						.unwrap_or((idx, ""));
					// 現在のスタック先頭と一致するか
					if let Some((_, start)) = stack.pop_if(|(x, _)| *x == tag) {
						// スタックが空なら反映
						if stack.is_empty() {
							let content = &raw[start..p];
							let content = match tag {
								"b" | "i" | "u" | "s" | "large" | "small" | "rainbow" => format!("<{0}>{1}</{0}>", tag, self.parse(content)),
								"ruby" => {
									let params = part(content, 1);
									match params.len() {
										2 => format!("<ruby>{}<rp>(</rp><rt>{}</rt><rp>)</rp></ruby>", self.parse(params[0]), self.parse(params[1])),
										_ => format!("<em>{}</em>", self.parse(content)),
									}
								}
								"image" => format!("<img src=\"{content}\">"),
								"" => {
									let params = part(content, 0);
									self.parse(params.choose(&mut rng).unwrap_or(&"")).into()
								}
								_ => format!("[{0}/{1}/{0}]", tag, self.parse(content)),
							};
							out.push_str(&content);
							end = idx + 1;
						}
						// 空じゃないならなにもしない（スタックを解消したところで満足し、中身の処理は↑で再帰的に行う）
					}
					// 先頭以外も一致するかを確認する処理にすればタグの交差を処理できるけど、よくわからなかったので一旦保留
				}
				b'\\' => {
					// エスケープ
					if let Some((_, b)) = bytes.next_if(|(_, b)| matches!(b, b'[' | b']' | b'|' | b'/' | b'\\')) {
						// とりあえず読み飛ばし、ネストが無いときのみ出力
						if stack.is_empty() {
							out.push_str(&raw[end..idx]);
							out.push(b as char);
							end = idx + 2;
						}
						// ネストがある場合は出力処理を最終的な再帰に任せる
					}
				}
				_ => (),
			}
		}
		if end > 0 {
			out.push_str(&raw[end..]);
			Cow::Owned(out)
		} else {
			Cow::Borrowed(raw)
		}
	}
}
