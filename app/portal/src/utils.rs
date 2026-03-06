pub mod password {
	use argon2::{
		Argon2, PasswordHasher as _, PasswordVerifier as _,
		password_hash::{Error, Result, SaltString, rand_core::OsRng},
	};

	pub fn hash(password: &str) -> Result<String> {
		Ok(Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))?.to_string())
	}

	pub fn verify(password: &str, hashed: &str) -> Result<bool> {
		match Argon2::default().verify_password(password.as_bytes(), &argon2::PasswordHash::new(&hashed)?) {
			Ok(_) => Ok(true),
			Err(Error::Password) => Ok(false),
			Err(err) => Err(err),
		}
	}
}

pub mod path {
	pub fn resource(path: &str) -> String {
		if cfg!(debug_assertions) {
			format!("{}/resource/{path}", crate::APP_PATH)
		} else {
			format!("{}/{path}", crate::APP_PATH)
		}
	}
}
