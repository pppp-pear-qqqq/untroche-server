mod auth;
mod entry;
mod profile;
mod report;
mod user;

use actix_cors::Cors;
use actix_web::web;

pub fn cfg(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("auth").wrap(Cors::default().allow_any_origin().allow_any_header()).configure(auth::cfg));
	cfg.service(web::scope("entry").configure(entry::cfg));
	cfg.service(web::scope("user").configure(user::cfg));
	cfg.service(web::scope("profile").configure(profile::cfg));
	cfg.service(web::scope("report").configure(report::cfg));
}
