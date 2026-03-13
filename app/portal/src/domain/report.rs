use actix_web::{HttpResponse, Responder, mime, web};
use chrono::Local;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::utils::{MessageResult, Name, PageResult, Template};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).post(post));
}

// 画面表示
async fn index() -> PageResult<impl Responder> {
	let html = Template::Base { summary: None }.render("html/report.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 投稿
#[derive(Deserialize)]
struct Post {
	body: String,
}
async fn post(web::Form(info): web::Form<Post>, user: Option<Name>, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	let timestamp = Local::now().timestamp();
	let user = user.as_deref();
	sqlx::query!("INSERT INTO report(timestamp,user,body) VALUES(?,?,?)", timestamp, user, info.body)
		.execute(pool.as_ref())
		.await?;
	Ok(HttpResponse::NoContent().finish())
}
