use std::{str::FromStr, sync::RwLock};

use actix_web::{HttpRequest, HttpResponse, Responder, error::*, web};
use sqlx::SqlitePool;

use crate::utils::{MessageResult, STATE, State};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.route("", web::to(async || "Hello Admin"));
	// GETメソッドでサーバー状態を変更するの行儀が悪いけど、適当に<a>並べるの楽だったので
	// Adminはどうせ自分しか見ないので多少行儀の悪い書き方しててもいいんじゃないだろうか
	cfg.route("state", web::to(state));
}

async fn state(req: HttpRequest, state: web::Data<RwLock<State>>, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	let new = State::from_str(req.query_string()).map_err(|_| ErrorBadRequest("無効なステートが指定されました"))?;
	let mut guard = state.write().map_err(|_| ErrorInternalServerError("アプリケーション状態読み込みに失敗"))?;
	println!("{:?} -> {:?}", guard, new);
	*guard = new;
	drop(guard);
	let str = new.to_string();
	sqlx::query!("UPDATE setting SET value=?2 WHERE key=?1", STATE, str).execute(pool.as_ref()).await?;
	Ok(HttpResponse::NoContent().finish())
}
