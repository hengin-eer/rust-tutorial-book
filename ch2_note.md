## Cargoについて
Rustのビルドシステムであり、パッケージマネージャーの機能も持ち合わせている。
コマンドラインでプロジェクトの操作が可能。

|コマンド|機能|
|---|---|
|`cargo new <project-name>`|プロジェクトの雛形作成|
|`cargo build`|プロジェクトのビルド|
|`cargo run`|実行|
|`cargo check`|文法チェック|
|`cargo test`|プログラムのテスト|
|`cargo doc`|ドキュメントの生成|
|`cargo publish`|ライブラリの公開|

一般的なJSフレームワークについているようなコマンドもあれば、ドキュメントの生成、ライブラリの公開など、馴染みのないものもある。
ライブラリの公開は [crates.io](https://crates.io) へとされる。

Cargoプロジェクトでは基本的に`cargo run`コマンドを使用すればいい。このコマンドでビルドと実行の2つが同時で行われるため、`cargo build`を使用しなくても良いのである。

## TOML
プロジェクトを作成すると、`Cargo.toml`ファイルが生成される。
このファイルにはプロジェクトの名前やバージョンなどの基本情報・設定情報が記載されている。

TOMLは設定ファイルを記述するための言語で、YAMLなどに名前も似ているため想像はつく。

仕様はGitHub上で公開されているため、暇なとき、本格的に学習したい時に見てみると良いかも。
[TOMLの仕様書 - GitHub](https://github.com/toml-lang/toml.io)
[日本語訳 TOMLの仕様書 - GitHub](https://github.com/toml-lang/toml.io/blob/main/specs/ja/v1.0.0.md)

## クレート
Rustのライブラリをクレートと言う。
この章では公開されているクレートを使用したツールをいくつか作成する。

### クレートの追加方法
`Cargo.toml`の[dependencies]の下にクレート名とバージョンを記載するだけ。npmの`package.json`やFlutter`pubspec.yaml`に記述するのと一緒。

### 各クレートについて

#### `rand`
`gen_range`メソッドの引数は以下のように指定する。
```rust
gen_range(1..6)
// このように記述すると1から5までの範囲

gen_range(1..=6);
// このように記述すると1から6までの範囲
```

通常は未満になるが、"="を加えると範囲は"以下"になる。



## コラム
### Rustのコンパイラ、やっぱり親切やわ～
`BigInt`の代わりに`BitInt`とタイポしてしまったとき、コンパイラのメッセージで"help: a similar name exists in the module: `BigInt`"と教えてくれた。このおかげで平凡なタイポだったけどすぐに解決できた。
ありがとう、`rustc` 😍