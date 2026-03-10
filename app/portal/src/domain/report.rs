use actix_web::{HttpResponse, Responder, mime, web};

use crate::{
	types::{MessageResult, PageResult},
	utils::Template,
};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).post(post));
}

// 画面表示
async fn index() -> PageResult<impl Responder> {
	let html = Template::Base { summary: None }.render("html/report.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}

// 投稿
async fn post() -> MessageResult<impl Responder> {
	Ok("") // TODO
}
