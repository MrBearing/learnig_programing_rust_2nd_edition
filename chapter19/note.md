# 19章 並列性 を読んでのメモ書き

なんとなくだけど、並列プログラミングは怖い。。

## 19.1 フォーク・ジョイン並列

- フォーク/ジョインのモデルは様々な言語で実装されているので、大体わかるはず。。
- join完了後に結果の統合が必要になるので、よく考えて使う
- 分割可能な操作にしないとあまり意味がない
- 真面目にサンプル動かそうと考えると、loadとsave書かなきゃいけないな。。
### 19.1.1 spawnとjoin
- この辺なんとなく、”並行プログラミング入門”読んだときに書いてあったな。。
- fork/joinでよくやるスタイルなので素直に読んでよいか。。
- RustっぽくするならWorklistはイテレータにしてfor_eachとか？

### 19.1.2 スレッド間でのエラー処理
- パニックしたか/してないかがResultとしてわかる。
  - 凄い助かる
- Resultだからjoinをしてるスレッドに値を返せる
  - c++だったらすげぇややこしいのやらねばならぬ。。
  - 「あ〜、これ未定義動作ですね」と言われない

### 19.1.3 不変データのスレッド間共有

- 14章の「盗むクロージャ」はあとで見る
- まるごとコピーして子スレッドに渡すのは避けたい。
  - 状況いかんでは使うかも
- Arcを使えば不変データもスレッド間共有できる
  - あくまで参照なので元データをコピーしない
  - デッドロック起こすような参照の場合はMutexかな？

### 19.1.4 Rayon

- spawn/joinをうまいことやってくれる
- ”並列イテレータ”がイテレータと変わらない使い勝手
  - 処理の分割は適当にやってくれる
  - par_iter()カコいい

### 19.1.5 マンデルブロ集合再訪
- [x] あとでRayon使って再実装してみる
- スレッドのマネージメントはクレートにやらせよう
  - 75%も速度が改善したのはすごい。。

速度の改善結果確認

```bash
# in chaper 19 dir. using rayon
$ time target/debug/mandelbrot mandel_rayon.png 4000x3000 -1.20,0.35 -1,0.20
real    0m30.894s
user    1m45.340s
sys     0m0.025s
# in chapter02 dir. 
$ time target/debug/mandelbrot mandel_rayon.png 4000x3000 -1.20,0.35 -1,0.20

real    0m30.503s
user    1m48.222s
sys     0m0.033s

```
## 19.2 チャネル

### 19.2.1 値の送信
- [このリポジトリを参照](https://github.com/ProgrammingRust/fingertips)
- 公式ガイドでこの辺は見た。
### 19.2.2 値の受信
- 公式ガイドでこの辺は見た。
- main.rs L116辺りの内容
### 19.2.3 パイプラインの実行
- main.rs L216
- スレッドのJoinHandleとreceiverを返す関数を組み合わせる
  - この関数の型は一般化できるのでは？
  - きれいに書けるクレートとかありそうだけど。。。

### 19.2.4 チャネルの機能と性能
- mpscのチャンネルを共有したければMutex
- 早すぎる送信を強制的に遅くするにはsync_channelを使う
  - メモリ効率が改善するか確認
  - 。。maxの使用量かわらない？やり方悪いかも。
  - [ ] 再度確認

そのままの実行結果
```bash
$ /usr/bin/time -v ./target/debug/fingertips test_data/pg100.txt 
indexed document 0, 5757401 bytes, 991503 words
4071004 bytes main, 4885113 bytes total
wrote file "./tmp00000001.dat"
        Command being timed: "./target/debug/fingertips test_data/pg100.txt"
        User time (seconds): 1.94
        System time (seconds): 0.01
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:01.96
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 36432
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 11121
        Voluntary context switches: 22
        Involuntary context switches: 34
        Swaps: 0
        File system inputs: 0
        File system outputs: 9544
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
```

sync_channel(32)を使用した場合

```bash
$ /usr/bin/time -v ./target/debug/fingertips test_data/pg100.txt 
indexed document 0, 5757401 bytes, 991503 words
4071004 bytes main, 4885113 bytes total
wrote file "./tmp00000001.dat"
        Command being timed: "./target/debug/fingertips test_data/pg100.txt"
        User time (seconds): 1.94
        System time (seconds): 0.04
        Percent of CPU this job got: 100%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:01.99
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 36352
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 11118
        Voluntary context switches: 23
        Involuntary context switches: 25
        Swaps: 0
        File system inputs: 0
        File system outputs: 9544
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0
```

Releaseじゃないからか？
### 19.2.5 スレッド安全性 : SendとSync
- SendやSyncはマーカートレイトなので、勝手に実装できるのでは？
  - ~~実際にはSendとして保証できない``` impl Send for XXX {} ``` とか書けば、実装できてしまうのでは？~~
  - [Rustnomiconで記述](https://doc.rust-lang.org/stable/nomicon/send-and-sync.html)が有るな。。
  - 実際には``` unsefe impl Send for XXX {}```と書くことになる
    - unsafeコードに対しての保証は自前でやるということだろう。

### 19.2.6 ほとんどすべてのイテレータをつなげられるチャネル
- "誓約と制約"かｗ。。

### 19.2.7 パイプライン以外のチャネルの使用法
- 例えばロギング
- 非同期の送受信
## 19.3 可変状態の共有
### 19.3.1 排他ロックとは何か？
- c++の排他ロックのサンプルコード何使ってるんだろ？
  - pthread じゃない
  - std::mutexっぽい
- 過去に一回派手にバグらせてから、ビビってる。
  - C++のMutexはデータとロックが別々になってるのホントにひどいと思う。

### 19.3.2 Mutex<T>
- 何時見てもMutexの中に値が格納できるの素敵


### 19.3.3 可変性とMutex
- Elon Muskおめでとう。次はTwitterだよな。
- &mut参照は排他アクセスになる
- 　

### 19.3.4 排他ロックがいつもいいとは限らないのはなぜか
- 良性の競合状態。。？
  - ”並行プログラミング入門”だと競合状態(race condition)は起こすなって書いてたけど。。
- 競合状態の定義が曖昧？
- 排他ロックがやばいのはなんとなくわかる
- モノリシックな書き方になるのは避けたい

### 19.3.5 デッドロック
- デッドロック怖い
- RustのMutexでデッドロックは**防げない**
- お決まりのクリティカルセクションを小さく保つ/メソッドを跨がない等の対策が必要

### 19.3.6 毒された排他ロック
- poisonedは致命的ではない
  - 多分、死ぬまでに猶予が有る/毒された部位を取り除けば生き延びる/適切に解毒すれば生き延びる等の意味合いが有ると思われ。
- MutexへのlockでErrが出ると以降そのMutexへのlockはエラーを返し続ける

### 19.3.7 排他ロックを用いた、複数の消費者を持つチャネル
- [ ] プログラムは実装してテストしてみる
- channelのReciverはMutexで包めば共有できる
- ```Arc<Mutex<Reciver<T>>>```とか必要だけど、冷静に読み解こうね
　　

### 19.3.8 リードライトロック（RwLock<T>）
- ここからはMutex以外のstd::syncについての話
  - "The Book"だとこの辺の話おざなりだった
- 読み出しは多数だけど、書き込みできる人は一人のR/W
- read,writeそれぞれがロックを掛けに行く
### 19.3.9 条件変数（Condvar）
- pthreadで使ったこと有る
- Condvar::waitで待機,notify_allで待機スレッドに通知

### 19.3.10 アトミック変数
- いわゆる原子性(R/W等の一連の操作が途中で終了することなく成功か失敗のどちらかになること)を保証する型
- [ ] サンプルはどうにかして実行してみる


### 19.3.11 グローバル変数
- 一般にグローバル変数は便利だけど、トラブルの元
  - Rustではどうなんだろう？
- アトミックなグローバル変数は単純な整数や真偽値のみ
  - 他の型でアトミックなグローバル変数を作る場合は以下の2つをクリアする必要が有る
    - スレッドセーフにする
    - static初期化時には、constで修飾した関数のみ使用可
  - [ ] コード実装して動かせるか確認
  - 

## 19.4 Rustでの並列コードの開発
- スレッドはできる限り隔離
- ”いくつかのテクニック”=フォーク/ジョイン、チャネル、ロック
- コンパイラと議論して頑張って説得するのがRustのスタイル
