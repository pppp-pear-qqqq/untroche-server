use actix_web::{Responder, web};

use crate::types::{MessageResult, PageResult};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").get(index).post(post));
}

// 画面表示
async fn index() -> PageResult<impl Responder> {
	Ok("")
}

// 投稿
async fn post() -> MessageResult<impl Responder> {
	Ok("")
}
