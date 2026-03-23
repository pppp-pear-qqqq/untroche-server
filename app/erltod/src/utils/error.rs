use actix_web::{HttpResponse, mime};

use super::Template;

common::error!(pub MessageError);
impl actix_web::ResponseError for MessageError {
	fn status_code(&self) -> actix_web::http::StatusCode {
		self.status_code
	}

	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code()).content_type(mime::TEXT_PLAIN).body(format!("{self}"))
	}
}

common::error!(pub PageError);
impl actix_web::ResponseError for PageError {
	fn status_code(&self) -> actix_web::http::StatusCode {
		self.status_code
	}

	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		let mut builder = HttpResponse::build(self.status_code());
		let tpl = Template::Base {
			nobots: true,
			summary: None,
			user: None,
		};
		let main = format!("{self}");
		match tpl.render_raw(&main) {
			Ok(html) => builder.content_type(mime::TEXT_HTML).body(html),
			Err(err) => builder.content_type(mime::TEXT_PLAIN).body(format!("{main}\n\n\n(liquid error)\n{err}")),
		}
	}
}

pub type MessageResult<T> = Result<T, MessageError>;
pub type PageResult<T> = Result<T, PageError>;
