# 16章を読んでの覚書
## 16.1 概要

## 16.2 Vec<T>
- VecDeque<T>は初めて知った
- LinkedList<T>はあまり使われない

### 16.2.1 要素へのアクセス
- 

### 16.2.2 イテレート処理

### 16.2.3 ベクタの伸長と縮小
### 16.2.4 連結

### 16.2.5 分割

### 16.2.6 入れ替え

### 16.2.7 フィル
### 16.2.8 ソートと検索
### 16.2.9 スライスの比較
### 16.2.10 ランダムな要素
### 16.2.11 Rustでは無効化エラーは生じない

## 16.3 VecDeque<T>

## 16.4 BinaryHeap<T>

## 16.5 HashMap<K, V>とBTreeMap<K, V>

### 16.5.1 エントリ

- map.entry()を使うと
- Entry<'a, K, V>の定義
- イテレータは以下で生成可能
  - keys , values, values_mut,
  - into_iter等はマップを消費してイテレータを返す 


### 16.5.2 マップに対するイテレート

## 16.6 HashSet<T>とBTreeSet<T>

### 16.6.1 セットのイテレート

### 16.6.2 値が等しいが別のものの場合

### 16.6.3 セット全体に対する演算

## 16.7 ハッシュ

## 16.8 ハッシュアルゴリズムのカスタマイズ
- std:: hash::BuildeHasherを使うとHashアルゴリズムを自前実装できる
- デフォルトはSipHash-1-3
- 安全性を犠牲にFowler-Noll-Voハッシュを使うことも可能

## 16.9 標準コレクションを超えて
