# Topcoat 0.5.0 調査メモ

移植を始める前に調べた事実とハマりどころ。**答えではなく地図**として使う。

調査日: 2026-08-03 / 対象バージョン: topcoat 0.5.0（crates.io 最新）

## 前提

- リポジトリ: https://github.com/tokio-rs/topcoat
- **early-stage / experimental。破壊的変更が入ると明言されている**
- サーバーレンダリング専用。**WASM は使わない**。`$(...)` 式は Rust のサブセットを
  マクロで JS にクロスコンパイルして、初回はサーバーで、以降はブラウザで再実行する
- 常駐 Tokio ランタイムが必要 → デプロイ先はコンテナ or VM

## 🔴 docs と実装が食い違う

**GitHub の main ブランチの docs は、crates.io の 0.5.0 と API が違う。**
実際に踏んだ例:

| main の docs | 0.5.0 の実際 |
| --- | --- |
| `topcoat::router::request::uri` | `topcoat::router::uri`（フラットな re-export） |
| `path_param!(slug);`（関数形式マクロ） | `#[path_param]` **属性マクロ**をタプル構造体に付ける |

**確実なのは、ダウンロード済みの実ソースを読むこと:**

```
~/.cargo/registry/src/*/topcoat-0.5.0/src/
~/.cargo/registry/src/*/topcoat-router-0.5.0/src/
~/.cargo/registry/src/*/topcoat-router-macro-0.5.0/docs/
```

`docs.rs/topcoat/0.5.0` はバージョンが固定されているので、GitHub の README より安全。

## セットアップ

```bash
cargo new portfolio
cargo add topcoat
cargo add tokio --features rt-multi-thread,macros
cargo install topcoat-cli      # dev サーバー / アセットバンドル / topcoat fmt
```

**`topcoat new` はまだ存在しない**（ロードマップ段階）。手で組む。

## CLI

```bash
topcoat dev             # ビルド + アセットバンドル + ファイル監視 + 再起動
                        # ページに topcoat::dev::script() があるとライブリロードする
                        # ターミナルで `r` を押すと手動リビルド
topcoat asset bundle    # 手動ビルド時のアセットバンドル（--release / --bin / --package）
topcoat asset list
topcoat asset clean
topcoat fmt             # view! マクロの中まで整形する
topcoat ui add <name>   # Topcoat UI のコンポーネントをプロジェクトにコピーする
```

`cargo topcoat ...` としても呼べる。

## ポート

`topcoat::start` は **`HOST` / `PORT` 環境変数**を見る。既定は `127.0.0.1:3000`。

```bash
HOST=0.0.0.0 PORT=8080 topcoat dev
```

SIGTERM / Ctrl+C でグレースフルシャットダウンする（in-flight リクエストを待つ）。

## アセットシステム

- `asset!("./foo.png")` を Rust コードに書くと、宣言がバイナリに埋め込まれる
- バンドラは**コンパイル済みバイナリを走査して**アセットを見つけ、
  `<cargo-target>/<profile>/assets` にコピー（リモート URL ならダウンロード）する
- ルータ側で `AssetBundle::load()` を `.build()` の前に呼ぶと、
  `/_topcoat/assets` 配下でコンテンツハッシュ付き URL として配信される
- `AssetBundle::load()` は**実行ファイルの隣の `assets/`** を探す

### 🔴 ハマりどころ

- **ハンドルが使われていないアセットは最適化で消える。** バイナリに残らないので
  バンドルにも入らない
- **バイナリとバンドルは必ず同じビルドから出す。** プロファイルが違うだけでも不一致になる
  （アセット ID が `OUT_DIR` のパス＝ target ディレクトリ + プロファイル + ビルドハッシュ に依存するため）
- バンドルに無いアセットをレンダリングすると**パニックする**。
  これは「ビルド/デプロイの不整合」を意味するので、握りつぶさないこと

## Tailwind 連携

- `build.rs` から `topcoat::tailwind::BuildConfig::new().render()` を呼ぶ
- Node も PostCSS も Vite も使わない。**Tailwind 標準 CLI（既定 4.3.2）を
  GitHub からダウンロード**して実行する（`target/topcoat/cache/tailwind` にキャッシュ）
- 出力は `$OUT_DIR/tailwind.css`。これが普通の Topcoat アセットになる
- レイアウトからは `tailwind::stylesheet!()` で参照する
- `dependencies` と `build-dependencies` の**両方**で `tailwind` feature を有効にする
  （build 側は `default-features = false`）
- カスタム CSS を使うなら `.input("src/styles/global.css")`

### 🔴 ハマりどころ

- **クラス走査は `.gitignore` に従う。** `/target` の行が無いとビルド成果物まで走査して、
  遅いうえに前のビルドのクラスが復活する
- **クラス名を実行時に組み立てると Tailwind から見えない。** 静的な文字列で書くこと
- オフラインビルドが必要なら `.executable("tailwindcss")` か
  `.executable_env("TAILWIND_CLI")` で既存の CLI を使う（ダウンロードが走らなくなる）
- `render()` は `rerun-if-*` を出さないので、Cargo の既定（パッケージ内の
  非 ignore ファイルが変わったら再実行）が効く。自分で `rerun-if-*` を書くと**上書きされる**

## ルーティング

- 明示パス: `#[page("/users/{id}")]`、catch-all は `/docs/{*path}`、
  グループは `/(marketing)/pricing`（URL からは消えるがレイアウト適用には効く）
- `module_router!()` はモジュールツリーから URL を導出する。
  **ルートツリーの根に置く**こと
- `#[layout]` はページ（またはネストしたレイアウト）を `Result` として受け取る
- 動的セグメント: 非ルートのルートモジュール内に `#[path_param]` を宣言すると、
  そのモジュールのセグメント自体がパラメータになる。
  `PostId` → `{post_id}`、`Slug` → `{slug}`
- **1 モジュールにつき `#[path_param]` は 1 つ**（1 モジュール = 1 セグメントなので）

### パスパラメータの読み方

- 値はハンドラの引数ではなく **`Cx` から読む**（`path_param::<T>(cx)`）。
  レイアウトやヘルパー関数からも同じ値が読める
- 内側の型が `str` ならパース無しで `&str`。それ以外は `FromStr` で
  `Result<&T, &Err>` になる
- `#[path_param(error = not_found)]` のように書くと、パース失敗が
  そのままルータのエラーレスポンスになり `?` で投げられる
- パースはリクエストごとに1回だけ実行され、メモ化される

## リクエストコンテキスト（`Cx`）

0.5.0 ではフラットに re-export されている: `topcoat::router::{parts, method, uri, version, headers, content_type, extensions}`

（main ブランチではこれらが `router::request` 配下に移動している）

- ミドルウェアではなく**普通の関数**で認証などのリクエストスコープの関心事を表現するのが推奨
- `#[memoize]` でリクエストごとのキャッシュ / fan-out の重複排除ができる

## 未実装（ロードマップ）— 自分で埋める必要があるもの

このポートフォリオの移植に直接効くもの:

- [ ] **静的エクスポート / プリレンダリング**
- [ ] **Markdown サポート** ← 作品データがこれ
- [ ] **画像最適化・リサイズ** ← 旧 `astro:assets` の webp + srcset に相当するものが無い
- [ ] **サイトマップ**
- [ ] `topcoat new`
- [ ] クライアントサイドナビゲーション / プリフェッチ
- [ ] ストリーミング SSR / Suspense
- [ ] Islands
- [ ] バリデーション、ローカライゼーション、認証、バックグラウンドジョブ

## 参考リンク

- README（各機能のガイドへのリンク集）: https://github.com/tokio-rs/topcoat#learn-topcoat
- Getting started: https://github.com/tokio-rs/topcoat/blob/main/crates/topcoat/docs/getting_started.md
- 公式サンプル 25 個: https://github.com/tokio-rs/topcoat/tree/main/examples
  - 特に `module-router` / `tailwind` / `asset` / `runtime` / `shard` が参考になる
- Discord（tokio）: https://discord.gg/tokio
