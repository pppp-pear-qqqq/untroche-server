use std::{
	future::{Ready, ready},
	ops::Deref,
};

use actix_session::SessionExt as _;
use actix_web::FromRequest;
use serde::de::DeserializeOwned;

pub const KEY: &str = "login-session";

// ログイン必須な関数の引数
pub struct Identity<T>(pub T);
// ログイン情報を受け取れる関数の引数
pub struct OptionalIdentity<T>(pub Option<T>);

impl<T> Deref for Identity<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> Deref for OptionalIdentity<T> {
	type Target = Option<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: DeserializeOwned> FromRequest for Identity<T> {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
		ready(match req.get_session().get(KEY) {
			Ok(Some(v)) => Ok(Self(v)),
			Ok(None) => Err(actix_web::error::ErrorUnauthorized("ログインしてください")),
			Err(err) => Err(actix_web::error::ErrorBadRequest(err)),
		})
	}
}

impl<T: DeserializeOwned> FromRequest for OptionalIdentity<T> {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
		ready(match req.get_session().get(KEY) {
			Ok(Some(v)) => Ok(Self(Some(v))),
			Ok(None) => Ok(Self(None)),
			Err(err) => Err(actix_web::error::ErrorBadRequest(err)),
		})
	}
}
