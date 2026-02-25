use std::env;

use actix_web::{App, HttpServer, web};
use sqlx::{Connection, SqliteConnection};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
	let load_env = |path: &str| env::var(path).expect(&format!("`{path}` is undefined"));
	// 環境変数読み込み
	let host = load_env("SERVER_HOST");
	let port = load_env("SERVER_PORT");
	let db_url = load_env("DATABASE_URL");
	let _ = db(&db_url);

	// サーバー構築
	HttpServer::new(|| App::new().service(web::resource("/").to(async || "Hello World")))
		.bind(format!("{host}:{port}"))?
		.run()
		.await
}

async fn db(url: &str) -> Result<(), sqlx::Error> {
	let mut db = SqliteConnection::connect(&url).await?;
	let a = sqlx::query!("SELECT * FROM user").fetch_all(&mut db).await?;
	Ok(())
}
