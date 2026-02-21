use std::{
	future::{Ready, ready},
	ops::Deref,
	sync::RwLock,
};

use actix_web::{FromRequest, web};

pub trait IsMaintenance: Clone + PartialEq {
	fn is_maintenance(&self) -> bool;
}
#[derive(Debug)]
pub struct State<T>(T);
impl<T: IsMaintenance> State<T> {
	pub fn eq_any<const N: usize>(&self, args: [T; N]) -> bool {
		args.contains(&self.0)
	}
	pub fn is_none<const N: usize>(&self, args: [T; N]) -> bool {
		!args.contains(&self.0)
	}
	pub fn change(&mut self, value: T) -> T {
		let old = self.0.clone();
		self.0 = value;
		old
	}
}

impl<T> From<T> for State<T> {
	fn from(value: T) -> Self {
		Self(value)
	}
}
impl<T> Deref for State<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: IsMaintenance + 'static> FromRequest for State<T> {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
		ready((|| {
			let state = req
				.app_data::<web::Data<RwLock<State<T>>>>()
				.ok_or(actix_web::error::ErrorInternalServerError("アプリケーション状態が未定義"))?;
			let guard = state.read().map_err(|_| actix_web::error::ErrorInternalServerError("アプリケーション状態読み込みに失敗"))?;
			let state = guard.0.clone();
			drop(guard);
			if state.is_maintenance() {
				Err(actix_web::error::ErrorForbidden("メンテナンス中"))
			} else {
				Ok(State(state))
			}
		})())
	}
}
