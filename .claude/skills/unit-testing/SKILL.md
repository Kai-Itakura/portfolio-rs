---
name: unit-testing
description: このリポジトリの Rust コード（特に src/domain/ のドメイン層）にユニットテストを書く・直す・足すときの決まりと手順。テストの置き場所・名前・書き方、条件網羅（C2）を基準にしたテストケースの選び方、cargo-llvm-cov による網羅率の表示までを扱う。「テストを書いて」「テストを足して」「網羅率は？」「この関数を確かめて」など、テストやカバレッジに触れる依頼では必ずこのスキルを使う。
---

# ユニットテスト

ドメインの関数は純粋なので、入力を作って呼び、出力を確かめるだけでテストできる。
テストの目的は、`docs/domain-model.md` に書いた規則が、本当にコードで守られていることを確かめること。

## 書く前に読むもの

- `lint/src/rules/` — このリポジトリ独自の静的解析のルール。1 ファイルが 1 ルールで、テスト関数の名前の決まり
  （`test_name.rs`）なども入っている。書くテストはこれらのルールに違反しないようにする

## 置き場所

テストする関数と同じファイルの末尾に置く。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // テスト
}
```

同じファイルに置くと、非公開の関数もテストでき、関数とテストを一緒に読める。

## 名前

英語の `snake_case` で、何を確かめるかを書く。形は `<関数>_<結果>_when_<条件>` を基本にする。
**期待する結果を必ず名前に入れる。** `mark_removed_upstream_from_published` のように入力だけを書いた名前では、
テストが落ちたときに何が壊れたのか分からないため。

- `publish_fails_when_removed_upstream`
- `publish_keeps_published_when_already_published`
- `from_dates_returns_ongoing_when_end_is_none`

## 書き方

Arrange（準備）→ Act（実行）→ Assert（確認）の 3 段に分け、段の間を空行で区切る。

```rust
#[test]
fn publish_fails_when_removed_upstream() {
    let project = professional_project(Publication::RemovedUpstream);

    let result = project.publish();

    assert!(matches!(result, Err(PublishError::RemovedUpstream)));
}
```

- Act は 1 つの関数呼び出しにして、結果を変数で受ける。準備と実行を 1 行にまとめたり（`let result = project(...).publish();`）、
  呼び出しを `assert!` の中に書いたりしない。3 段が見て分かれていると、何を準備し何を確かめたかが読み取りやすいため
- 実行に渡す引数の値も、準備の段で変数に作っておく。`project.add_work(work(...))` のように実行の行で値を作らず、
  `let work = work(...);` と準備したうえで `project.add_work(work)` と呼ぶ
- 1 つのテストで確かめるのは 1 つの振る舞い
- `Result` や `enum` は `matches!` で確かめる。テストのためだけにドメインの型へ `PartialEq` などの derive を足さない。
  足したほうがよいと思ったら、理由を添えて確認する
- 成功した値の中身を確かめたいときは `let Ok(value) = result else { panic!("expected Ok, got an error") };` で取り出す
- テスト用の値を作る補助関数（例: `fn professional_project(publication: Publication) -> Project<Professional>`）は、
  `tests` モジュールの中に置く。どこまで共有するかはその都度判断し、テストを読んだだけで入力が分かる程度にとどめる
- 固定の ULID や日付は、補助関数の中で作ってよい（`NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()` など）。
  テストの中の `unwrap()` は、失敗したらテストが落ちるだけなので使ってよい

## どのテストを書くか: 条件網羅（C2）

基準は**条件網羅（C2）**。判定に出てくる**個々の条件**が、それぞれ真にも偽にもなるテストを書く。

1. テストする関数を読み、判定を拾う: `if`・`&&`・`||` の各条件、`if let`、`match` のガード（`if …`）、`match` の各腕
2. それぞれの条件が真になる入力と、偽になる入力を用意する。`a && b` なら、`a` の真・偽と `b` の真・偽が少なくとも 1 回ずつ出るようにする
3. 加えて、成功（`Ok`）と、**エラーの種類ごと**の失敗を少なくとも 1 つずつ書く。`match` の腕もすべて通す
4. `docs/domain-model.md` の「不変条件のありか」の表と照らし、「関数（…）」で守っている規則にテストが無いものがないか確かめる
5. 型で守っている規則（例: 実務の案件に `add_work` が無い）は、コンパイルが通らないことが保証なのでユニットテストは書かない

検査の順番に意味がある関数（例: 消えた案件で、すでに作品もある）は、どちらのエラーが返るかを確かめるテストも書く。

## 実行と網羅率の表示

テストを書いたら実行し、網羅率を表示する。

```bash
cargo test
cargo +nightly llvm-cov --branch --summary-only
cargo run -p lint
```

`cargo run -p lint` で、`lint/src/rules/` のルールに違反していないことを確かめる。
自分が書いたテストの違反は直す。触っていないコードの違反は直さずに報告する。

網羅率の測定には nightly の Rust と `cargo-llvm-cov` が要る（ビルドや `cargo test` は stable のまま）。
入っていなければ入れる:

```bash
rustup toolchain install nightly --profile minimal -c llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

`--branch` の表の読み方:

| 列 | 意味 |
| --- | --- |
| Branches | `if`・`&&`・`||` の各条件、`if let`、`match` のガードが、真と偽のそれぞれを通ったか。**C2 はこの列で見る** |
| Lines / Regions | 実行された行・区間。`match` の腕は Branches に出ないので、通っていない腕はここで見つける |
| Functions | 1 回でも呼ばれた関数 |

足りない条件を見つけるには、ファイルを絞って詳細を出す:

```bash
cargo +nightly llvm-cov --branch --text 2>/dev/null | grep -B2 -A4 "Branch ("
```

`Branch (行:列): [True: n, False: 0]` のように 0 がある条件は、その側を通るテストが無い。
実行回数が `0` の行は、通っていない `match` の腕や関数。

### 報告の形

テストを書いて実行したら、最後に次を表示する。

1. 足したテストの一覧（名前と、確かめた条件）
2. `cargo test` の結果（通った数・落ちた数）
3. `cargo run -p lint` の結果（最後の 1 行と、違反があればその一覧）
4. テストした範囲の網羅率の表（`--summary-only` の、対象ファイルの行と TOTAL）
5. C2 で足りない条件があれば、その場所（ファイル:行）と理由。わざとテストしないもの（型で守っている規則など）はそう書く
6. 網羅率が 100% でなければ、その理由。多くはテストの側の `let ... else` や `matches!` のガードの、テストが落ちたときだけ通る側。
   TOTAL に、今回テストしていないファイルが含まれているならそう書く。
   テストする関数の条件がすべて真と偽を通っているかを、表の数字とは分けて書く

ドメインの型がまだどこからも使われていないと、`cargo test` で「使われていない」という警告が出ることがある。
テストの結果とは関係ないので、警告が出たことだけ報告する。

アプリ全体がビルドできない環境（`build.rs` がネットワークから取ってくるものが取れないなど）では、
`src/domain/` だけを別のクレートにコピーしてテストと網羅率の測定を行い、そうしたことを報告する。
`cargo run -p lint` はアプリをビルドしないので、その環境でも走らせられる。
