use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
	Active,
	Close,
	Maintenance,
}

impl common::IsMaintenance for State {
	fn is_maintenance(&self) -> bool {
		*self == State::Maintenance
	}
}
impl ToString for State {
	fn to_string(&self) -> String {
		match self {
			Self::Active => "active",
			Self::Close => "close",
			Self::Maintenance => "maintenance",
		}
		.into()
	}
}
impl FromStr for State {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"active" => Self::Active,
			"close" => Self::Close,
			"maintenance" => Self::Maintenance,
			_ => return Err(()),
		})
	}
}
