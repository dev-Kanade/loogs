# LoOgs

**LoOgs(ローグス)**はRustプロジェクトでデバッグやログ、エラーを日時と分かりやすく出力するためのクレートです。

## インストール

プロジェクトにLoOgsを追加するために以下のコマンドを実行してください。
```cargo
cargo add loogs
```
または
`Cargo.toml`の`dependencies`に以下を追加
```toml
[dependencies]
loogs = "1.0.3"
```

## ドキュメント
まず、利用する場所で`use`しないといけません。
```rust
use loogs;
```

**LoOgs**には4つのマクロがあります。

### INFO
これは通常の`println!`や`print`と同じ動作をします。
```rust
