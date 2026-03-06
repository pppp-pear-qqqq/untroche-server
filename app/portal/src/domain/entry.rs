use actix_session::Session;
use actix_web::{HttpResponse, Responder, error::*, mime, web};
use serde::Deserialize;
use sqlx::SqlitePool;
use validation::Validation;

use crate::types::{Id, MessageResult, PageResult, State, StateHandle};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).post(login).delete(logout));
	cfg.service(web::resource("register").post(register));
}

// ログイン
#[derive(Deserialize, Validation)]
struct Authorize {
	#[validation(name = "ユーザー名", min = 2, max = 20)]
	name: String,
	#[validation(name = "パスワード", min = 8)]
	password: String,
}

// エントランス画面
async fn index() -> PageResult<impl Responder> {
	Ok(HttpResponse::Ok()
		.content_type(mime::TEXT_HTML)
		.body(std::fs::read_to_string("app/portal/resource/html/register.html")?))
}

async fn login(info: web::Form<Authorize>, session: Session, _: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	let hashed = sqlx::query_scalar!("SELECT password FROM user WHERE name=?", info.name).fetch_one(pool.as_ref()).await?;
	if crate::utils::password::verify(&info.password, &hashed).map_err(|err| ErrorInternalServerError(err))? {
		Id::save(&session, &info.name)?;
		Ok(HttpResponse::NoContent().finish())
	} else {
		Err(ErrorUnauthorized("ユーザー名またはパスワードが異なります").into())
	}
}

// ログアウト
async fn logout(session: Session) -> MessageResult<impl Responder> {
	Id::delete(&session);
	Ok(HttpResponse::NoContent().finish())
}

// 新規登録
async fn register(info: web::Form<Authorize>, session: Session, state: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	if *state != State::Active {
		return Err(ErrorForbidden("当サイトはクローズしています").into());
	}
	let hashed = crate::utils::password::hash(&info.password).map_err(|err| ErrorInternalServerError(err))?;
	match sqlx::query!("INSERT INTO user(name,password) VALUES(?,?)", info.name, hashed).execute(pool.as_ref()).await {
		Ok(_) => {
			Id::save(&session, &info.name)?;
			Ok(HttpResponse::NoContent().finish())
		}
		Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(ErrorConflict("ユーザー名が重複しています").into()),
		Err(err) => Err(err.into()),
	}
}
