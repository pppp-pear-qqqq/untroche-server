use actix_web::{HttpResponse, Responder, error::*, mime, web};
use base64::{Engine, prelude::*};
use chrono::Local;
use rand::{TryRngCore as _, rngs::OsRng};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
	types::{MessageResult, Name, State, StateHandle},
	utils::Template,
};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(issue).post(cert));
}

async fn issue(name: Option<Name>, state: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	// 認証コード有効期限(秒)
	const EXPIRY: i64 = 120;

	if *state != State::Active {
		return Err(ErrorForbidden("当サイトはクローズしています").into());
	}
	let code = if let Some(name) = name {
		// コード生成
		let mut dst = [0xffu8; 20];
		OsRng.try_fill_bytes(&mut dst)?;
		let code = BASE64_URL_SAFE_NO_PAD.encode(dst);
		let timestamp = Local::now().timestamp() + EXPIRY;
		sqlx::query!("INSERT INTO auth(code,timestamp,user) VALUES(?,?,?)", code, timestamp, *name)
			.execute(pool.as_ref())
			.await?;
		Some(code)
	} else {
		None
	};
	// コードを埋め込んだhtmlを返す
	let html = Template::None.render(
		"auth.html",
		liquid::object!({
			"code": code.as_ref(),
		}),
	)?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

#[derive(Deserialize)]
struct Cert {
	code: String,
}
async fn cert(info: web::Json<Cert>, state: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	if *state != State::Active {
		return Err(ErrorForbidden("当サイトはクローズしています").into());
	}
	let pool = pool.as_ref();
	let timestamp = Local::now().timestamp();
	sqlx::query!("DELETE FROM auth WHERE timestamp<?", timestamp).execute(pool).await?;
	match sqlx::query_scalar::<_, String>("SELECT user FROM auth WHERE code=?").bind(&info.code).fetch_one(pool).await {
		Ok(id) => Ok(HttpResponse::Ok().body(id)),
		Err(sqlx::Error::RowNotFound) => Err(ErrorUnauthorized("認証コードが不正です").into()),
		Err(err) => Err(err.into()),
	}
}
