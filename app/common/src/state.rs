use std::{
	future::{Ready, ready},
	ops::Deref,
	sync::RwLock,
};

use actix_web::{FromRequest, error::*, web};

pub trait IsMaintenance {
	fn is_maintenance(&self) -> bool;
}

/// # Example
/// ```
/// let app_data = web::Data::new(common::StateHandle::new(State::Active).pack());
/// ```
pub struct Handle<T: Clone + IsMaintenance>(T);

impl<T: Clone + IsMaintenance> Handle<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
	pub fn pack(self) -> RwLock<T> {
		RwLock::new(self.0)
	}
}
impl<T: Clone + IsMaintenance> Deref for Handle<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: Clone + IsMaintenance + 'static> FromRequest for Handle<T> {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
		ready((|| {
			let state = req.app_data::<web::Data<RwLock<T>>>().ok_or(ErrorInternalServerError("アプリケーション状態が未定義"))?;
			let guard = state.read().map_err(|_| ErrorInternalServerError("アプリケーション状態読み込みに失敗"))?;
			let state = guard.clone();
			drop(guard);
			if !state.is_maintenance() {
				Ok(Handle(state))
			} else {
				Err(ErrorForbidden("メンテナンス中"))
			}
		})())
	}
}
