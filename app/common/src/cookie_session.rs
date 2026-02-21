use std::{env, fs, io::Write as _};

use actix_session::{SessionMiddleware, config::PersistentSession, storage};
use actix_web::cookie;
use base64::prelude::*;

pub fn get_cookie_key(name: &str) -> cookie::Key {
	if let Ok(var) = env::var(name) {
		cookie::Key::from(&BASE64_STANDARD.decode(var).expect("failed decode var"))
	} else {
		let key = cookie::Key::generate();
		if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(".env") {
			let _ = writeln!(&mut f, "{name}={}", BASE64_STANDARD.encode(&key.master()));
		}
		key
	}
}
pub fn get_cookie_session(key: cookie::Key, path: String) -> SessionMiddleware<storage::CookieSessionStore> {
	SessionMiddleware::builder(storage::CookieSessionStore::default(), key)
		.cookie_path(path)
		.cookie_secure(false)
		.cookie_http_only(false)
		.session_lifecycle(PersistentSession::default().session_ttl(cookie::time::Duration::days(14)))
		.build()
}
