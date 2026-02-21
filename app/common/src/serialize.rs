pub fn sep_digit<S: serde::Serializer>(value: &i64, s: S) -> Result<S::Ok, S::Error> {
	let value = value.to_string();
	let len = value.len();
	let mut ret = String::new();
	for (i, ch) in value.chars().enumerate() {
		ret.push(ch);
		if (len - (i + 1)) % 3 == 0 && (i + 1) != len {
			ret.push(',');
		}
	}
	s.serialize_str(&ret)
}
pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub fn timestamp<S: serde::Serializer>(v: &chrono::DateTime<chrono::Local>, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_str(&v.format(DATETIME_FORMAT).to_string())
}
