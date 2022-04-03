# 15章を読んでの覚書

- 「速度よりもl表現力を重視する関数型プログラミング」という表現は使えそう
- 

##  15.1 IteratorトレイトとIntoIteratorトレイト

- Iterator はnextを持つだけの単純なトレイト
- IntoIteratorはinto_iterメソッドでIteratorを生成できるトレイト
- 用語の整理
  - iterable: IntoIteratorを実装した型
  - consumer: イテレータを受け取るコードのこと
 
##  15.2 イテレータの作成

### 15.2.1 iterメソッドとiter_mutメソッド

- 使い方の話だけっぽい？
- iter_mut()使ってないっぽいけど？
  - 内部の値を入れ替えたりだからよいのかな

### 15.2.2 IntoIteratorの実装

- 可変参照に対するIntoIteratorを実装していない型があるから注意
- ジェネリックなコード書くときにIntoIteratorをWhere区で取ると便利

### 15.2.3 from_fnとsuccessors

- from_fn は関数で指定したイテレータを作る関数
  - クロージャの引数はなし？指定可能？
    - [ ] 要調査　API見ればわかるはず
- successors 一個前に生成された値を使用するして等で値を生成する場合
- クロージャ内に複雑な関数も書けるけど、多用は厳禁
  - 関数型の書き方の良さを潰してしまう使い方もありうる
  - 例えばクロージャ内部でフィルター書いちゃうとかやると台無し
- 本文中にもあるけど、できるだけ使用を避けたほうが良い関数

### 15.2.4 drainメソッド

- 使いどころは多そう
- drainが返したイテレータの所有権はコンシューマに移る
- drainしたイテレータがドロップすると中身は削除されるので、単純な削除にも使える

### 15.2.5 他のイテレータの生成方法

- 一覧だけなので読んで終わり
  - おぼえるの無理
  - 覚える必要はないでしょう多分
##  15.3 イテレータアダプタ

Iteratorトレイトはアダプタメソッドをいくつか持ってるので、この章ではそれらを解説

### 15.3.1 mapとfilter

- よくあるmap/filterの動作
- 戻り値 はstd::iter::Map / ::Filter
  - この類の書き方よく見かけるけどIteratorトレイトで返さず、一旦別のトレイトにしてるのか謎
  - あるいは継承してる？
    - [ ] 理由・定義を確認する
  - Rust的なあるいは型システム的にこういう書き方のほうがいいのか？

### 15.3.2 filter_mapとflat_map
- filter_map() はok()などのオプションを返すクロージャの中身がSome(X)ならXを収集して、Noneの場合は捨てる動作
- flat_mapは配列を持ってるstruct等の配列要素全てになにかしたいとき等便利
  - クロージャの戻り値がIntoIterなら大丈夫
### 15.3.3 flatten
- 配列の配列とかをばらしてイテレータにしてくれる
- flattenはOptionもばらしてSome(X)だけ取り出してくれる
  - ちょっと副作用的になりそうだから注意したほうがいいかも。
- v.s. flat_mapになりがち 
- [ ] 3重の場合どうなる? 動作をチェック
  - 多分Vec<>のイテレータが返ってくる気がする。。 

### 15.3.4 takeとtake_while

- take 指定された回数で停止するイテレータ
- take_while クロージャでfalseが出るまで値を取り続ける
- 
### 15.3.5 skipとskip_while
- 指定した回数読み飛ばしをする _whileはクロージャが**true**になるまで値をスキップ

### 15.3.6 peekable
- peek()を使ってイテレータの中身を覗ける。
- peek()を使うにはPeekableである必要がある
- Peekableにするにはpeekable()を使うだけ

### 15.3.7 fuse
- ヒューズみたいに切れたら(=Noneを出したら)復旧しないようにする
- 出処の怪しいイテレータにはfuse()しておく

### 15.3.8 反転可能イテレータとrev
- DoubleEndedIteratorとrevの話
  - スタックとかキューとか自前実装するときべんりかな。
  - ↑すでにあるかも。。。
### 15.3.9 inspect
- イテレータの要素をinspect (＝検査)するためのアダプタ
  - クロージャを渡してprintln!やassert_eq!等の処理はできるが,値の更新はできない

### 15.3.10 chain
- そのまま、2つのイテレータの接続に用いる

### 15.3.11 enumerate


### 15.3.12 zip
### 15.3.13 by_ref
### 15.3.14 clonedとcopied
### 15.3.15 cycle
##  15.4 イテレータの消費
### 15.4.1 単純な累積 : count、sum、product
### 15.4.2 max、min
### 15.4.3 max_by、min_by
### 15.4.4 max_by_key、min_by_key
### 15.4.5 アイテム列の比較
### 15.4.6 any、all
### 15.4.7 position、rposition、ExactSizeIterator
### 15.4.8 foldとrfold
### 15.4.9 try_foldとtry_rfold
### 15.4.10 nthとnth_back
### 15.4.11 last
### 15.4.12 find、rfind、find_map
### 15.4.13 コレクションの作成 : collectとFromIterator
### 15.4.14 Extendトレイト
### 15.4.15 partition
### 15.4.16 for_eachとtry_for_each
##  15.5 ユーザ定義イテレータの実装
