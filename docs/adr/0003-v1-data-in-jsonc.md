# v1 のデータは JSONC に置き、作品の本文だけ Markdown にする

v1 の案件・技術・作品のデータは `data/v1.jsonc` に書き、コンパイル時に `include_str!` で埋め込んで、
起動時にインメモリ DB へ読み込む。作品の本文だけは `content/<作品 ID>.md` に置き、同じく `include_str!` で埋め込む。
画像は `ImageKey`（画像のファイル名）で指し、`asset!` の定数への対応はページ側に書く。
実行時のファイル I/O は無い。

データを JSONC にしたのは、コメントを書けて手で直しやすく、別システムとつなぐまでの仮置きとして、
後で DB に替えても形を引き継げるため。本文を Markdown のファイルに残したのは ADR 0001 と同じ理由で、
日本語の散文は文字列リテラルより別ファイルのほうが直しやすく、diff も読みやすいため。

## Consequences

- 1 つの作品の情報は 3 か所に分かれる。JSONC の `work`、`content/<作品 ID>.md` と
  `src/infrastructure/v1_data.rs` の `WORK_BODIES` の 1 行、`assets/` の画像とページ側の `asset!` の対応
- `include_str!` も `asset!` もリテラルのパスしか受け付けないので、作品 ID や画像のファイル名から
  パスを組み立てられない。対応を表に 1 行ずつ書くのはこのため
- 本文の書き忘れは起動時に分かる（`WORK_BODIES` に無ければ読み込みが失敗する）。
  画像の対応の書き忘れをどこで検出するかは、ページを作るときに決める
- JSONC の形は `src/infrastructure/v1_data.rs` の受け取り用の型で決まる。ドメインの型は JSONC を知らない
