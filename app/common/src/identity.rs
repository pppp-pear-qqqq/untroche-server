use std::{
	future::{Ready, ready},
	ops::Deref,
};

use actix_session::{Session, SessionExt as _};
use actix_web::{FromRequest, error::*};
use serde::{Serialize, de::DeserializeOwned};

pub const KEY: &str = "login-session";

pub struct Identity<T: DeserializeOwned>(T);

impl<T: DeserializeOwned + Serialize> Identity<T> {
	pub fn save(session: &Session, value: &T) -> Result<(), actix_session::SessionInsertError> {
		session.insert(KEY, value)
	}
	pub fn delete(session: &Session) {
		session.remove(KEY);
	}
}
impl<T: DeserializeOwned> Deref for Identity<T> {
	type Target = T;

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
			Ok(None) => Err(ErrorUnauthorized("ログインしてください")),
			Err(err) => Err(ErrorBadRequest(err)),
		})
	}
}
