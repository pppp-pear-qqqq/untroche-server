common::error!(pub MessageError);
impl actix_web::ResponseError for MessageError {
	fn status_code(&self) -> actix_web::http::StatusCode {
		self.status_code
	}

	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		actix_web::HttpResponse::build(self.status_code()).content_type(actix_web::mime::TEXT_PLAIN).body(format!("{self}"))
	}
}

common::error!(pub PageError);
impl actix_web::ResponseError for PageError {
	fn status_code(&self) -> actix_web::http::StatusCode {
		self.status_code
	}

	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		actix_web::HttpResponse::build(self.status_code()).content_type(actix_web::mime::TEXT_PLAIN).body(format!("{self}"))
	}
}

pub type MessageResult<T> = Result<T, MessageError>;
pub type PageResult<T> = Result<T, PageError>;
