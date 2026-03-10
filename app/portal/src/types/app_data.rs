use std::{str::FromStr as _, sync::RwLock};

use actix_web::{cookie, web};
use base64::{Engine, prelude::BASE64_STANDARD};
use sqlx::SqlitePool;

use super::{State, StateHandle};

#[derive(Clone)]
pub struct AppData {
	pub pool: web::Data<SqlitePool>,
	pub state: web::Data<RwLock<State>>,
	pub session_key: cookie::Key,
	pub admin_key: String,
}
impl AppData {
	pub async fn new(url: &str) -> Self {
		// キー定義
		const STATE: &str = "STATE";
		const KEY: &str = "KEY";
		// DB接続
		let pool = SqlitePool::connect(url).await.unwrap();
		// State読み込み
		let state = match sqlx::query_scalar!("SELECT value FROM setting WHERE key=?", STATE).fetch_one(&pool).await {
			Ok(r) => State::from_str(&r).unwrap(),
			Err(sqlx::Error::RowNotFound) => {
				let state = State::Maintenance;
				let str = state.to_string();
				sqlx::query!("INSERT INTO setting VALUES(?,?)", STATE, str).execute(&pool).await.unwrap();
				state
			}
			Err(err) => panic!("{}", err),
		};
		// Key読み込み
		let (session_key, admin_key) = match sqlx::query_scalar!("SELECT value FROM setting WHERE key=?", KEY).fetch_one(&pool).await {
			Ok(r) => (cookie::Key::from(&BASE64_STANDARD.decode(&r).unwrap()), r),
			Err(sqlx::Error::RowNotFound) => {
				let session_key = cookie::Key::generate();
				let admin_key = BASE64_STANDARD.encode(&session_key.master());
				sqlx::query_scalar!("INSERT INTO setting VALUES(?,?)", KEY, admin_key).execute(&pool).await.unwrap();
				(session_key, admin_key)
			}
			Err(err) => panic!("{}", err),
		};
		println!("admin: {admin_key}");
		// 作成
		Self {
			pool: web::Data::new(pool),
			state: web::Data::new(StateHandle::new(state).pack()),
			session_key,
			admin_key,
		}
	}
}
