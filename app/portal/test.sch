setting
	columns
		key text pri
		value text

user
	columns
		name text pri
		password text

auth
	columns
		code text pri
		timestamp timestamp
		user text ref(user.name) update(cascade) delete(cascade)
