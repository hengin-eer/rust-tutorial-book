## 基本文法

関数の宣言は`fn`にて行う。

`println!`マクロ（っていうらしい。関数じゃないの？）内の文字列に変数を埋め込むにはC言語とやり方が似ている。
しかし微妙に変数の置き方が違い、Rustでは文字列中の変数を`{}`と置く。C言語の場合、変数の型によって`%d`や`%f`などを適切に使用する。

### 型について
Rustは型に厳密な言語、と書籍で紹介されているが、今のところ変数宣言時に明確な型定義は行っていない。
ただし、変数に代入する値の型に注意している。
例えば、数値型について小数点を指定するかしないかで、指定しないと「整数型」、指定すると「実数型」になる。

上記のように変数の値によって自動的に型が決まる**型推論機能**を使って今までコードを書いてきたけど、いよいよ変数型について習得する。
まず数値型からだが、
1. 符号あり整数
2. 符号なし整数
3. 浮動小数点数

の３つを扱う。
それぞれ使用ビット数によって使い分ける必要があって、符号あり整数で説明すると、`i8`, `i16`, `i32`, `i64`, `i128`, `isize`がある。
符号なし整数の場合は`u8`から`usize`まであり、浮動小数点数は`f32`, `f64`がある。
ちなみに`isize`や`usize`は使っているOSのビット数に依存するから、自分の場合は`i64`, `u64`と同じ。

### for文
基本的に`n`回から`N`回ループ処理を繰り返す時、以下のように記述できる。
```rust
for i in n..N {
    // 処理を記述する
}
```

文法はC言語と言うよりかはPythonなどの今風に近い感じ。（C言語で同様の記述が出来るけど知らないだけかも）

また、ループ文には欠かせない条件文を()で囲ったりすると、コンパイル時に警告されるらしい。
実行は可能だけれど、コーディングルールみたいなものらしい。ちなみに自分は不満。

`else if`の書き換えとして、1個前の`if`文内の処理で`continue;`と書くことで、次の条件分岐に`if`を使うことが出来る。

応用として`map`を使ってベクターを生成するなどがあるらしい。他にも変数型やクロージャ―などの知識が今後必要になってくる。

### 変数への再代入禁止
Rustでは基本、変数の値は書き換えられない（immutable）ようになっている。
値を書き換えられる変数は`mut`（mutable）で定義できる。
