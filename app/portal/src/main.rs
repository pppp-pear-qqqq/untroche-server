use std::env;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
	let load_env = |path: &str| env::var(path).expect(&format!("`{path}` is undefined"));
	// 環境変数読み込み
	let host = load_env("SERVER_HOST");
	let port = load_env("SERVER_PORT");
	// let db_url = load_env("DATABASE_URL");

	// サーバー構築
	HttpServer::new(|| App::new().service(web::resource("/").to(async || "Hello World")))
		.bind(format!("{host}:{port}"))?
		.run()
		.await
}
