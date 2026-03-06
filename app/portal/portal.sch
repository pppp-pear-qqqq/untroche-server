user
	id text primary
	password text
	profile text default('')		# プロフィール、プレイヤーのSNSアカウントやキャラクターなど　全部ひとまとめにする　未エスケープ
	webhook text null	# 共通のウェブフックURL
	mutes blob			# ユーザーミュートのリスト(Vec<id>)

auth
	code text primary
	timestamp timestamp	# 期限となるタイミングを保持する
	user text ref(user.id) update(cascade) delete(cascade)

bbs
	id int primary
	timestamp timestamp
	user text null ref(user.id) update(cascade) delete(setnull)
	address text		# ポータル側で各アプリから参照するBBSを管理してもいい（どのみち読み込みは動的なので）リクエスト返却時にCORSを良くする
	body text			# 後から編集とかやらないので投稿時にエスケープ処理する

