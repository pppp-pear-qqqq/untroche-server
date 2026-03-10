use actix_web::{HttpResponse, Responder, mime, web};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

use crate::{
	types::{MessageResult, PageParams, PageResult},
	utils::Template,
};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(list).post(search));
	cfg.service(web::resource("{id}").get(index));
}

#[derive(Deserialize)]
struct Search {
	#[serde(flatten)]
	page: PageParams,
	name: Option<String>,
}

// ユーザ一覧
async fn list(_info: web::Query<Search>) -> PageResult<impl Responder> {
	let html = Template::Base { summary: None }.render("html/user/list.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 検索API
async fn search(info: web::Json<Search>, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	#[derive(FromRow, Serialize)]
	struct Record {
		id: String,
		profile: String,
	}
	let mut builder = sqlx::QueryBuilder::new("SELECT * FROM user");
	if let Some(name) = &info.name {
		let search = format!("%{name}%");
		builder.push(" WHERE name LIKE ?").push_bind(search);
	}
	builder
		.push(" ORDER BY name ASC LIMIT ")
		.push_bind(info.page.offset() as i64)
		.push(",")
		.push_bind(info.page.limit() as i64);
	let result: Vec<Record> = builder.build_query_as().fetch_all(pool.as_ref()).await?;
	Ok(HttpResponse::Ok().content_type(mime::APPLICATION_JSON).body(serde_json::to_string(&result)?))
}

// プロフィール表示
async fn index() -> PageResult<impl Responder> {
	let html = Template::Base { summary: None }.render("html/user/index.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}
