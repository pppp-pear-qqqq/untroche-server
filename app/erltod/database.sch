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
	icon text expr(icons.lines[0])	# これ成立しないのでちゃんと定義する

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
	# TODO 戦闘設定で具体的に何するかを思い出す

table navigator	# 場合によってはactor_styleと統合する？　明確に中身が違いそうだから分けるべきではあるが、世界観的には同じもの
	@pk(eno,name)
	eno ref(actor.eno).update(cascade).delete(cascade)
	name text
	words blob	# タイミング・発言のHashMap
