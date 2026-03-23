use actix_web::{HttpResponse, Responder, error::*, mime, web};
use serde::Deserialize;
use sqlx::SqlitePool;
use validation::Validation;

use crate::utils::{Eno, MessageResult, PageResult, State, StateHandle, Template};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).patch(patch).delete(delete));
	cfg.service(web::resource("battle").get(battle::index).post(battle::post).delete(battle::delete));
}

// 編集・設定画面
async fn index() -> PageResult<impl Responder> {
	let html = Template::Base {
		nobots: true,
		summary: None,
		user: None,
	}
	.render("html/profile/index.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 更新処理
#[derive(Deserialize, Validation)]
struct Patch {
	#[validation(name = "キャラクター名", max = 30, min = 1)]
	name: Option<String>,
	#[validation(name = "1行コメント", max = 30)]
	comment: Option<String>,
	#[validation(name = "プロフィール", max = 4000)]
	profile: Option<String>,
	#[validation(name = "プロフィール画像", max = 2000)]
	portraits: Option<String>,
	#[validation(name = "アイコン画像", max = 2000)]
	icons: Option<String>,
}
async fn patch(web::Json(info): web::Json<Patch>, eno: Eno, state: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	fn format_urls(v: String) -> String {
		let mut out = String::new();
		for line in v.lines() {
			let line = line.trim();
			if !line.is_empty() {
				if !out.is_empty() {
					out.push('\n');
				}
				out.push_str(line);
			}
		}
		out
	}
	if *state != State::Active {
		return Err(ErrorForbidden("当サイトはクローズしています").into());
	}
	// SQL構築
	let mut builder = sqlx::QueryBuilder::new("UPDATE actor SET ");
	let mut sep = builder.separated(',');
	// 接続
	let pool = pool.as_ref();
	// 名前
	if let Some(v) = info.name {
		sep.push("name=").push_bind_unseparated(v);
	}
	// コメント
	if let Some(v) = info.comment {
		sep.push("comment=").push_bind_unseparated(v);
	}
	// プロフィール
	if let Some(v) = info.profile {
		// DBに保持するのは生データ、表示時にエスケープ
		sep.push("profile=").push_bind_unseparated(v);
	}
	// プロフィール画像
	if let Some(v) = info.portraits {
		sep.push("portraits=").push_bind_unseparated(format_urls(v));
	}
	// アイコン画像
	if let Some(v) = info.icons {
		sep.push("icons=").push_bind_unseparated(format_urls(v));
	}
	builder.push(" WHERE name=").push_bind(&*eno);
	builder.build().execute(pool).await?;
	Ok(HttpResponse::NoContent().finish())
}

// キャラクター削除
#[derive(Deserialize)]
struct Delete {
	eno: i64,
	user: String,
}
async fn delete(web::Form(info): web::Form<Delete>, eno: Eno, _: StateHandle, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	if info.eno != *eno {
		return Err(ErrorForbidden("Enoが正しくありません").into());
	}
	let pool = pool.as_ref();
	let user = sqlx::query_scalar!("SELECT user FROM actor WHERE eno=?", *eno).fetch_one(pool).await?;
	if info.user != user {
		return Err(ErrorForbidden("ユーザー名が正しくありません").into());
	}
	sqlx::query!("DELETE FROM actor WHERE eno=?", *eno).execute(pool).await?;
	Ok(HttpResponse::NoContent().finish())
}

mod battle {
	use actix_web::{HttpResponse, Responder, mime};

	use crate::utils::{MessageResult, PageResult, Template};

	pub(super) async fn index() -> PageResult<impl Responder> {
		let html = Template::Base {
			nobots: true,
			summary: None,
			user: None,
		}
		.render("html/profile/battle.html", liquid::object!({}))?;
		Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
	}

	pub(super) async fn post() -> MessageResult<impl Responder> {
		Ok(HttpResponse::NoContent().finish())
	}

	pub(super) async fn delete() -> MessageResult<impl Responder> {
		Ok(HttpResponse::NoContent().finish())
	}
}
