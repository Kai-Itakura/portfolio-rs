---
name: domain-layer
description: このリポジトリのドメイン層（src/domain/ 以下の Rust コード）を書く・直すときの決まりと手順。案件（Project）・技術（Technology）・作品（Work）の型や関数を足す・変える、規則（不変条件）を足す、エラー型を足す、ID や値オブジェクトを足す、ドメインの設計を docs/domain-model.md や CONTEXT.md に合わせる、といった作業では、ファイル名や「ドメイン」という言葉が出てこなくても必ずこのスキルを使う。
---

# ドメイン層の実装

このリポジトリのドメイン層は『Domain Modeling Made Functional』（Scott Wlaschin、以下 DMMF）の考え方で書いている。
ドメインの型と規則は、このサイトで唯一「間違ってはいけない」部分なので、規則はできるだけ型で守り、
型で守れないものは生成と変更の関数で守る。

## 書く前に読むもの

- `CONTEXT.md` — 用語の定義。型や関数の名前はここの言葉に合わせる
- `docs/domain-model.md` — 型・関数・データの持ち主・不変条件のありか・未決事項。いまの設計の正
- `docs/adr/` — 設計判断の理由。特に別システムとの契約（ADR 0002）と v1 のデータ（ADR 0003）
- いま触るファイルと、その周りの既存コード。書き方は既存コードに合わせる

依頼が文書と食い違うとき、文書だけでは決められないときは、推測で埋めずに書く前に確認する。
オーナーは Rust を学んでいるので、決めるべきことは選択肢と理由を添えて聞く。

## 型の決まり

- **型と関数を分ける。** 型はデータだけを持ち、振る舞いは型を受け取って型を返す関数にする
- **「A か B か」は直和型（`enum`）、「A と B」は直積型（`struct`）。** 2 つの真偽値で表すと書けてしまう不正な組み合わせ
  （例: 「消えたのに公開している」）は、1 つの `enum` にして書けなくする（`Publication` がその例）
- **ID は newtype にする。** `pub struct XxxId(Ulid);` と `impl From<Ulid> for XxxId`。中身が同じ ULID でも別の型にして取り違えを防ぐ。
  derive は既存の ID（`src/domain/project/project_id.rs`）に揃える
- **種類によって持てる操作が違うものは、型で分ける。** 「個人開発だけが作品を持てる」は `Project<K: ProjectKind>` と
  `impl Project<Personal>` で表している。`ProjectKind` は封じたトレイト（sealed trait）なので、種類を足すのは
  `src/domain/project/project_kind.rs` の中だけでできる
- **失敗しうる生成はスマートコンストラクタにする。** `Result<Self, XxxError>` を返す関数で検査する（`Period::from_dates` が例）。
  型で守れない規則は、`domain-model.md` の不変条件の表に「関数（…）」として書く
- **ドメインは外を知らない。** `serde`・Topcoat の型（`Asset` など）・JSONC・HTTP・DB をドメインに持ち込まない。
  外から届いた形をドメインの型にする翻訳は `src/infrastructure/` に書く
- **ファイルは主な型の名前にする。** 集約の値オブジェクトは集約のフォルダに置く（`src/domain/project/period.rs` など）。
  モジュールは `project.rs` と `project/` を並べる形で、`mod.rs` は使わない

## 関数の決まり

関数はできるだけ純粋にする。`&mut self` で書き換えず、値を受け取って新しい値を返す。

```rust
pub fn publish(self) -> Result<Self, PublishError> {
    if self.publication == Publication::RemovedUpstream {
        return Err(PublishError::RemovedUpstream);
    }
    Ok(Self {
        publication: Publication::Published,
        ..self
    })
}
```

- 状態を変える関数は `self` を受け取り、`Self { 変える項目, ..self }` で新しい値を返す
- 読むだけの関数（検査など）は `&self` を受け取る（`ensure_deletable` が例）
- 失敗すると渡した値は返らないが、失敗したものは保存し直さないので困らない
- メソッドにするか、型と並べた関数にするかは既存コードに合わせる（いまはメソッド）

## エラーの決まり

- 失敗しうる関数ごとに、その関数のエラーの `enum` を持つ（`PublishError`、`AddWorkError` など）。
  バリアントは「なぜ失敗したか」を表す名前にする
- 検査の順番に意味があるときは、`domain-model.md` の関数の説明に順番を書く
- 層をまたいだエラーの包み方（`Display`・`std::error::Error`・`From` の実装）は、まず手で `impl` を書く方針。
  `thiserror` は使わない。ドメインのエラーにどこまで実装するかが決まっていないときは確認する

## 文書を合わせる

型や規則を変えたら、同じ変更の中で文書も直す。コードと文書が食い違ったままにしない。

- `docs/domain-model.md`
  - 「型」の節（DMMF の記法: `data X = a: A AND b: B`、`X OR Y`）
  - 「型どうしの関係」の図
  - 「関数」の節のシグネチャと失敗の説明
  - 「不変条件のありか」の表（型で守るか、関数で守るか）
  - 決まっていないことは「未決」の節へ
- `CONTEXT.md` — 用語が増えた・意味が変わったときだけ。定義だけを書き、実装の話は書かない
- 決定が「戻しにくい・理由が分からないと驚く・本当に選択肢があった」の 3 つを満たすなら ADR を提案する

## 確かめる

1. `cargo clippy --all-targets -- -D warnings`
2. `rustfmt --edition 2024 --check` で触ったファイルを確かめる（`view!` を含むファイルは `topcoat fmt`）
3. ドメインの関数を足した・変えたら、`unit-testing` スキルに沿ってユニットテストを書く

アプリ全体がビルドできない環境（`build.rs` がネットワークから取ってくるものが取れないなど）では、
`src/domain/` だけを別のクレートにコピーしてビルドし、そう確かめたことを報告する。

## 報告する

- 何を、なぜそう書いたかを説明する（オーナーが読んで学べるように）
- 通らなかったこと、走らせられなかったことも、そのまま書く
- 依頼の範囲外で見つけた問題は直さずに報告する
- 文書と既存コードの食い違いを見つけたら、すべて挙げる
