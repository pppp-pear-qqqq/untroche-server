use std::future::{Ready, ready};

use actix_web::FromRequest;

pub enum Device {
	PC,
	Mobile,
}

impl FromRequest for Device {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
		let device = if let Some(ua) = req.headers().get("User-Agent").and_then(|x| x.to_str().ok()) {
			if ua.contains("Mobile") || ua.contains("iPhone") || ua.contains("Android") || ua.contains("iPad") || ua.contains("Windows Phone") {
				Device::Mobile
			} else {
				Device::PC
			}
		} else {
			// User-Agentを読み取れなかった
			Device::PC
		};
		ready(Ok(device))
	}
}
