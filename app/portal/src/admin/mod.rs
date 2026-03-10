use std::sync::RwLock;

use actix_web::{
	HttpRequest, HttpResponse, Responder,
	error::*,
	web::{self, ServiceConfig},
};
use serde::Deserialize;

use crate::types::MessageResult;

pub fn cfg(cfg: &mut ServiceConfig) {
	cfg.route("", web::to(async || "Hello"));
	cfg.route("state", web::to(state));
}

#[derive(Deserialize)]
struct State {
	state: crate::types::State,
}
async fn state(web::Query(info): web::Query<State>, req: HttpRequest) -> MessageResult<impl Responder> {
	println!("state change? ここでエラー");
	let state = req.app_data::<web::Data<RwLock<State>>>().ok_or(ErrorInternalServerError("アプリケーション状態が未定義"))?;
	let mut guard = state.write().map_err(|_| ErrorInternalServerError("アプリケーション状態読み込みに失敗"))?;
	println!("{:?} -> {:?}", guard.state, info.state);
	guard.state = info.state;
	Ok(HttpResponse::NoContent().finish())
}
