use actix_web::{HttpResponse, Responder, mime, web};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

use crate::utils::{MessageResult, PageParams, PageResult, Template, template::Summary};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(list).post(search));
	cfg.service(web::resource("{name}").get(index));
}

#[derive(Deserialize)]
struct Search {
	#[serde(flatten)]
	page: PageParams,
	name: Option<String>,
}

// ユーザ一覧
async fn list(web::Query(_info): web::Query<Search>) -> PageResult<impl Responder> {
	let html = Template::Base {
		nobots: false,
		summary: None,
		user: None,
	}
	.render("html/user/list.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 検索API
async fn search(web::Json(info): web::Json<Search>, pool: web::Data<SqlitePool>) -> MessageResult<impl Responder> {
	#[derive(FromRow, Serialize)]
	struct Record {
		id: String,
		profile: String,
	}
	let mut builder = sqlx::QueryBuilder::new("SELECT * FROM user");
	if let Some(name) = info.name {
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
#[derive(Deserialize)]
struct Index {
	name: String,
}
async fn index(path: web::Path<Index>, pool: web::Data<SqlitePool>) -> PageResult<impl Responder> {
	let name = path.into_inner().name;
	let profile = sqlx::query_scalar!("SELECT profile FROM user WHERE name=?", name).fetch_one(pool.as_ref()).await?;
	let html = Template::Base {
		nobots: false,
		summary: Some(Summary {
			title: name.clone(),
			desc: profile.chars().take(20).collect(),
			url: format!("user/{name}"),
			ogtype: "profile".into(),
			image: "http://untroche.com/image/ogp.png".into(),
			card: "summary".into(),
		}),
		user: None,
	}
	.render("html/user/index.html", liquid::object!({"name":&name,"profile":&profile}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}
