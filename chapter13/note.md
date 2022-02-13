 
# 13章を読んでの覚書

- 表13.1は非常に便利
- 以下のトレイトに関しては対応する賞を読むこと
  - Iteratorトレイト,IntoIteratorトレイト->15章
  - Hashトレイト->16章
  - SendとSync->19章
- 
## 13.1 Drop

いわゆるデストラクタの動作をするトレイト

Q1. dropメソッドは誰が呼んでいるのか。(「システムが」等の曖昧な回答ではなく)

## 13.2 Sized

- イマイチよくわかってないdyn型。おそらくdynamicの略。動的に型付けしたいのかな？
- 11章のトレイトオブジェクトの章参照のこと
- ファットポインタ?
- Sizedはデフォルト```struct SomeStruct<T>{}```と書くと```struct SomeStruct<T: Sized>{}```の意味
- ```struct SomeStruct<T: ?Sized>{}```



## 13.3 Clone


## 13.4 Copy

## 13.5 DerefとDerefMut

## 13.6 Default

## 13.7 AsRefとAsMut

## 13.8 BorrowとBorrowMut

## 13.9 FromとInto

## 13.10 TryFromとTryInto

## 13.11 ToOwned

## 13.12 BorrowとToOwnedの動作例 : つつましいCow
