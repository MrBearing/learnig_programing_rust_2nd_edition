# 15章を読んでの覚書

- 「速度よりもl表現力を重視する関数型プログラミング」という表現は使えそう
- 



##  15.1 IteratorトレイトとIntoIteratorトレイト

- Iterator はnextを持つだけの単純なトレイト
- IntoIteratorはinto_iterメソッドでIteratorを生成できるトレイト
- 


##  15.2 イテレータの作成

### 15.2.1 iterメソッドとiter_mutメソッド

### 15.2.2 IntoIteratorの実装

### 15.2.3 from_fnとsuccessors

### 15.2.4 drainメソッド
### 15.2.5 他のイテレータの生成方法

##  15.3 イテレータアダプタ

### 15.3.1 mapとfilter
### 15.3.2 filter_mapとflat_map
### 15.3.3 flatten
### 15.3.4 takeとtake_while
### 15.3.5 skipとskip_while
### 15.3.6 peekable
### 15.3.7 fuse
### 15.3.8 反転可能イテレータとrev
### 15.3.9 inspect
### 15.3.10 chain
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
