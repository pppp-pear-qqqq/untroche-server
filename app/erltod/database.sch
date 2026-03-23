table setting
	key text pk
	value text

table actor
	eno int pk
	user text
	name text
	comment text default('')
	profile text default('')	# タグ未処理
	portraits text
	icons text
	icon text expr(substr(icons,1,instr(icons||char(10),char(10))-1))

table timeline
	id int pk
	timestamp timestamp
	place text	# DMでもグループでも一括で名前管理、DMの場合は"DM:238-444"みたいな名前が自動初期設定される（変更可）
	actor ref(actor.eno).update(cascade).delete(setnull)	# システムメッセージ・削除されたキャラクターの発言はnull
	name text
	body text	# タグ処理済み
	visible bool default(TRUE)
	# 発言に対するリプライはアンカー(>>{id})、人に対するリプライはメンション(@{eno})で行う　どちらもbody内で使用可能な構文として

table timeline_actor	# 自分宛の発言一覧を後から取得するための中間テーブル　アンカーでもメンションでもここに登録される
	id int pk
	timeline ref(timeline.id).update(cascade).delete(cascade)
	actor ref(actor.eno).update(cascade).delete(cascade)

table actor_style
	@pk(eno,name)
	eno ref(actor.eno).update(cascade).delete(cascade)
	name text
	type text	# カンマ区切りかなにかでキーワードを列挙する　検索時はLIKE句を使う
	value blob	# Vec<(id,name,word)> または HashMap<timing,word>

view navigator
	s.eno
	s.name
	words s.value
	@from(actor_style s WHERE type='navigator')

table skill
	id int pk
	name text
	cost int
	effect text
