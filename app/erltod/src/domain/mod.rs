use actix_web::{HttpResponse, Responder, mime, web};

use crate::utils::{PageResult, Template};

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.route("", web::get().to(index));
	// cfg.service(web::scope("auth").wrap(Cors::default().allow_any_origin().allow_any_header()).configure(auth::cfg));
	// cfg.service(web::scope("entry").configure(entry::cfg));
	// cfg.service(web::scope("user").configure(user::cfg));
	// cfg.service(web::scope("profile").configure(profile::cfg));
	// cfg.service(web::scope("report").configure(report::cfg));
}

async fn index() -> PageResult<impl Responder> {
	let html = Template::Base {
		nobots: false,
		summary: None,
		user: None,
	}
	.render("html/index.html", liquid::object!({}))?;
	Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).body(html))
}
