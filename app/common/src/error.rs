/// ## Example
/// ```
/// error!(MyError);
///
/// impl actix_web::ResponseError for MyError {
/// 	fn status_code(&self) -> actix_web::http::StatusCode {
/// 		self.status_code
/// 	}
///
/// 	fn error_response(&self) -> actix_web::HttpResponse {
/// 		actix_web::HttpResponse::build(self.status_code())
/// 			.content_type(actix_web::mime::TEXT_PLAIN_UTF_8)
/// 			.body(actix_web::body::BoxBody::new(format!("{self}")))
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! error {
	($ty: ident) => {
		pub struct $ty {
			status_code: actix_web::http::StatusCode,
			cause: Box<dyn std::error::Error>,
		}
		impl std::fmt::Debug for $ty {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:?}", self.cause)
			}
		}
		impl std::fmt::Display for $ty {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.cause)
			}
		}
		impl<T: std::error::Error + 'static> From<T> for $ty {
			fn from(value: T) -> Self {
				let status_code = if let Some(actix) = (&value as &dyn std::error::Error).downcast_ref::<actix_web::Error>() {
					actix.as_response_error().status_code()
				} else if let Some(Some(actix)) = value.source().map(|x| x.downcast_ref::<actix_web::Error>()) {
					actix.as_response_error().status_code()
				} else {
					actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
				};
				Self {
					status_code,
					cause: Box::new(value),
				}
			}
		}
		impl $ty {
			fn new(cause: Box<dyn std::error::Error>) -> Self {
				let status_code = if let Some(actix) = cause.downcast_ref::<actix_web::Error>() {
					actix.as_response_error().status_code()
				} else {
					actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
				};
				Self { status_code, cause }
			}
		}
	};
}
