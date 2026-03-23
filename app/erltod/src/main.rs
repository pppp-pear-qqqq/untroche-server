mod admin;
mod domain;
mod utils;

use actix_session::{SessionMiddleware, config::PersistentSession, storage};
use actix_web::{App, HttpResponse, HttpServer, cookie, middleware, web};
use common::AdminGuardMiddleware;

const APP_PATH: &str = "app/portal";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// 環境変数読み込み
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
	let load_env = |path: &str| std::env::var(path).expect(&format!("`{path}` is undefined"));
	let host = load_env("SERVER_HOST");
	let port = load_env("SERVER_PORT");
	let db_url = load_env("DATABASE_URL");

	// 設定初期化・必要な変数の読み込み
	let app = crate::utils::AppData::new(&db_url).await;

	// サーバー構築
	let server = HttpServer::new(move || {
		let app = app.clone();
		let session = SessionMiddleware::builder(storage::CookieSessionStore::default(), app.session_key)
			.cookie_secure(false)
			.session_lifecycle(PersistentSession::default().session_ttl(cookie::time::Duration::days(14)))
			.build();
		App::new()
			.wrap(middleware::Logger::default())
			.wrap(middleware::NormalizePath::trim())
			.wrap(session)
			.default_service(web::to(|| HttpResponse::NotFound()))
			.app_data(app.pool)
			.app_data(app.state)
			.service(web::scope("admin").wrap(AdminGuardMiddleware(app.admin_key)).configure(admin::cfg))
			.configure(domain::cfg)
	});
	server.bind(format!("{host}:{port}"))?.run().await
}
