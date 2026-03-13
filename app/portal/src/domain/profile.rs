use actix_web::{HttpResponse, Responder, error::*, mime, web};
use common::Webhook;
use serde::Deserialize;
use sqlx::SqlitePool;
use validation::Validation;

use crate::utils::{MessageResult, Name, PageResult, State, StateHandle, Template};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).patch(patch).delete(delete));
}

// 編集・設定画面
async fn index() -> PageResult<impl Responder> {
	let html = Template::Base {
		nobots: true,
		summary: None,
		user: None,
	}
	.render("html/profile.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 更新処理
#[derive(Deserialize, Validation)]
struct Patch {
	password: Option<Password>,
	#[validation(name = "プロフィール", max = 4000)]
	profile: Option<String>,
	#[validation(name = "ウェブフックURL", max = 256)]
	webhook: Option<String>,
	// mutes: Option<Vec<String>>,
}
#[derive(Deserialize, Validation)]
struct Password {
	now: String,
	#[validation(name = "パスワード", min = 8)]
	new: String,
}
async fn patch(web::Json(info): web::Json<Patch>, user: Name, state: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	if *state != State::Active {
		return Err(ErrorForbidden("当サイトはクローズしています").into());
	}
	// SQL構築
	let mut builder = sqlx::QueryBuilder::new("UPDATE user SET ");
	let mut sep = builder.separated(',');
	// 接続
	let pool = pool.as_ref();
	// パスワード
	if let Some(password) = info.password {
		let hashed = sqlx::query_scalar!("SELECT password FROM user WHERE name=?", *user).fetch_one(pool).await?;
		if !crate::utils::password::verify(&password.now, &hashed).map_err(|err| ErrorInternalServerError(err))? {
			return Err(ErrorForbidden("パスワードが異なります").into());
		}
		let hashed = crate::utils::password::hash(&password.new).map_err(|err| ErrorInternalServerError(err))?;
		sep.push("password=").push_bind_unseparated(hashed);
	}
	// プロフィール
	if let Some(v) = info.profile {
		// DBに保持するのは生データ、表示時にエスケープ
		sep.push("profile=").push_bind_unseparated(v);
	}
	// ウェブフック
	if let Some(v) = info.webhook {
		// 値がある場合のみテスト後に確定、無ければNULLをセット
		if !v.is_empty() {
			// 値がある
			match Webhook::new("ウェブフックURLが登録されました。\nこのメッセージを受け取れていれば正しく設定できています。", "untroche", None)
				.send(&v)
				.await
			{
				Ok(_) => (),
				Err(err) => return Err(ErrorBadRequest(format!("ウェブフックのテスト送信に失敗しました:\n{err}")).into()),
			}
			sep.push("webhook=").push_bind_unseparated(v);
		} else {
			sep.push("webhook=NULL");
		}
	}
	builder.push(" WHERE name=").push_bind(&*user);
	builder.build().execute(pool).await?;
	Ok(HttpResponse::NoContent().finish())
}

// ユーザー削除
#[derive(Deserialize)]
struct Delete {
	password: String,
}
async fn delete(web::Form(info): web::Form<Delete>, user: Name, _: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	let pool = pool.as_ref();
	let hashed = sqlx::query_scalar!("SELECT password FROM user WHERE name=?", *user).fetch_one(pool).await?;
	if !crate::utils::password::verify(&info.password, &hashed).map_err(|err| ErrorInternalServerError(err))? {
		return Err(ErrorForbidden("パスワードが異なります").into());
	}
	sqlx::query!("DELETE FROM user WHERE name=?", *user).execute(pool).await?;
	Ok(HttpResponse::NoContent().finish())
}
