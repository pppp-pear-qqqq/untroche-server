/// 数値の3桁ごとにカンマを入れる
pub fn to_comma_string(value: &i64) -> String {
	let mut ret = String::new();
	if *value < 0 {
		ret.push('-');
	}
	let value = value.abs().to_string();
	let len = value.len();
	for (i, ch) in value.chars().enumerate() {
		ret.push(ch);
		if (len - i - 1) % 3 == 0 && i != len - 1 {
			ret.push(',');
		}
	}
	ret
}
/// 数値の3桁ごとにカンマを入れてシリアライズする
pub fn as_comma_string<S: serde::Serializer>(value: &i64, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_str(&to_comma_string(value))
}

/// 共通の日付時刻フォーマット
pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// 共通のフォーマットを使用して日付時刻をシリアライズする
pub fn as_datetime<S: serde::Serializer>(v: &chrono::DateTime<chrono::Local>, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_str(&v.format(DATETIME_FORMAT).to_string())
}
