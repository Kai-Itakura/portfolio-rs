# Topcoat 0.8.1 調査メモ

実装の前に確かめた事実とハマりどころ。**答えではなく地図**として使う。

調査日: 2026-09-20 / 対象バージョン: topcoat 0.8.1 / topcoat-cli 0.8.1 / rustc 1.98.1

## 前提

- リポジトリ: https://github.com/tokio-rs/topcoat
- **early-stage / experimental。破壊的変更が入ると明言されている**
- サーバーレンダリング専用。**WASM は使わない**
- 常駐 Tokio ランタイムが必要 → デプロイ先はコンテナ or VM
- **MSRV は 1.98。** 0.7.0 で上がった。ツールチェインが古いと `cargo add` が
  黙って古いバージョンを選ぶ（Cargo の MSRV 対応リゾルバの挙動）

## 🟢 一次情報はローカルにある

**crate 自体がガイドを同梱している。** docs.rs や GitHub を見に行く前にここを読む。

```
~/.cargo/registry/src/*/topcoat-0.8.1/
├── CHANGELOG.md          # 破壊的変更はここに [**breaking**] で明示される
├── README.md
├── docs/                 # 機能ごとのガイド 18 本
│   ├── getting_started.md  router.md  asset.md  tailwind.md
│   ├── font.md  runtime.md  view.md  context.md  ui.md  session.md
│   └── cookie.md  icon.md  mail.md  htmx.md  datastar.md ...
└── src/
```

サブクレートも同じ階層に展開されている。挙動の裏取りが要るときはこちら:

```
topcoat-router-0.8.1/    topcoat-view-0.8.1/    topcoat-asset-0.8.1/
topcoat-core-0.8.1/      topcoat-font-0.8.1/    topcoat-ui-0.8.1/
```

`-macro` と `-grammar` がペアで存在する。`view!` や `#[page]` の構文定義が
独立クレートになっているので、マクロの文法を確かめたいときは `-grammar` を見る。

## セットアップ

```bash
cargo add topcoat
cargo add tokio --features rt-multi-thread,macros
cargo install topcoat-cli        # dev サーバー / アセットバンドル / fmt / UI
```

**`topcoat new` は存在しない。** 手で組む。雛形は `docs/getting_started.md` にある。

### 🔴 CLI とライブラリのバージョンを揃える

`topcoat-cli` はライブラリと同じバージョン番号で公開されている。
**両方 0.8.1 に揃えること。** ずれると CLI が警告を出す。
バンドラはコンパイル済みバイナリを走査する仕組みなので、走査側と埋め込み側で
想定が食い違うと壊れる。

## CLI

```bash
topcoat dev             # ビルド + アセットバンドル + 監視 + 再起動
topcoat fmt             # view! マクロの中まで整形
topcoat asset list      # バイナリに埋まっているアセットのパス一覧
topcoat asset bundle    # アセットをディレクトリに書き出す
topcoat asset clean     # バンドルとビルドキャッシュを削除
topcoat ui init         # UI コンポーネントの導入準備
topcoat ui add <name>   # 既成 UI コンポーネントをプロジェクトにコピー
topcoat ui list         # レジストリのコンポーネントと導入状況
topcoat ui remove       # 追加した UI コンポーネントを削除
```

`cargo topcoat ...` としても呼べる。

- `dev` / `asset bundle` は `--bin` / `-p` / `-r` / `--profile` を取る（`cargo build` と同じ）
- `asset bundle` は `-o, --out <DIR>` で出力先を変えられる
- `fmt` は `--stdin` と `--macros <名前リスト>` を取る
- ページに `topcoat::dev::script()` があると `dev` がライブリロードする。
  ターミナルで `r` を押すと手動リビルド

## ポート

`topcoat::start` は **`HOST` / `PORT` 環境変数**を見る。既定は `127.0.0.1:3000`。

```bash
HOST=0.0.0.0 PORT=8080 topcoat dev
```

`serve` / `serve_until` / `start` が `topcoat` 直下に re-export されている。
SIGTERM / Ctrl+C でグレースフルシャットダウンする。

## ルーティング

`docs/router.md`（445 行）が一次情報。

- 属性マクロ: `#[page("/users/{id}")]` / `#[layout("/")]` / `#[layer("/")]` /
  `#[route(GET "/api/health")]`
- catch-all は `/docs/{*path}`、グループは `/(marketing)/pricing`
- 登録方法は 3 つ:
  1. `module_router!()` — モジュールツリーから URL を導出する。**推奨**。
     `RouterBuilder` を返すので、続けて `.discover()` を呼ぶか手で足す
  2. `Router::builder().discover().build()` — `discover` feature が
     注釈付きアイテムをリンク時に集める
  3. 手動登録 — `.page(...)` `.layout(...)` などを列挙
- `#[layout]` はページ（またはネストしたレイアウト）を `Slot` として受け取る

### パスパラメータ — 🔴 0.5.0 から形が変わった

**`path_param!` は関数形式マクロ**（0.6.0 の breaking change #242）。
0.5.0 の属性マクロ形式ではない。

```rust
path_param!(post_id: u64, error = bad_request);
```

- Pascal ケースの型を生成する。`post_id` → `PostId`（`{post_id}` に対応）
- 値はハンドラ引数ではなく **`Cx` から読む**（`path_param::<PostId>(cx)`）。
  レイアウトやヘルパー関数からも同じ値が読める
- 型を書かなければ percent-decode 済みの `&str`。`:` で型を付けると `FromStr` で
  パースされ、既定の戻り値は `Result<&T, &<T as FromStr>::Err>`
- `error = bad_request` / `not_found` / `unauthorized` / `forbidden` /
  `redirect(...)` / `redirect_permanent(...)` でパース失敗をルータのエラーに写せる
- `*` を前置すると catch-all（`path_param!(*doc_path)`）
- `module_router!` では、**非ルートのルートモジュール内に宣言すると
  そのモジュールのセグメント自体がパラメータになる**
- パースはリクエストごとに 1 回でメモ化される

### クエリパラメータ

`#[query_params]` を名前付きフィールドの構造体に付けると `serde::Deserialize` が
derive され、`query_params::<T>(cx)` で読める。欠けうるキーは `Option<T>`。
こちらもリクエストごとに 1 回でメモ化される。

### リクエストコンテキスト（`Cx`）— 🔴 0.5.0 から形が変わった

**`topcoat::router::request` のネストしたモジュール**にある。
0.5.0 のフラットな re-export ではない。

```
topcoat::router::request::{parts, method, uri, version, headers, content_type, extensions}
```

いずれも `&Cx` を取る関数。

- ミドルウェアではなく**普通の関数**で認証などのリクエストスコープの関心事を
  表現するのが推奨（`docs/functions_not_middlewares.md`）
- `#[memoize]` でリクエストごとのキャッシュ / fan-out の重複排除ができる。
  0.6.0 から `as_ref` を手で指定する形に変わっている

## アセットシステム

- `asset!("./foo.png")` を Rust コードに書くと、宣言がバイナリに埋め込まれる
- バンドラは**コンパイル済みバイナリを走査して**アセットを見つけ、
  実行ファイルの隣の `assets/` にコピー（リモート URL ならダウンロード）する
- ルータ構築時に `.assets(AssetBundle::load().unwrap())` を **`.build()` の前に**呼ぶと、
  `/_topcoat/assets` 配下でコンテンツハッシュ付き URL として配信される
- `view!` の中に `Asset` を置くと、バンドル済みファイルの URL としてレンダリングされる
- 出力先を変えたときは `AssetBundle::load_dir("dist/assets")` で読む

### 🔴 ハマりどころ

- **ハンドルが使われていないアセットは最適化で消える。** バイナリに残らないので
  バンドルにも入らない
- **バイナリとバンドルは必ず同じビルドから出す。** プロファイルが違うだけでも不一致になる。
  アセット ID は宣言時のパスから導かれ、`tailwind::stylesheet!()` のような
  ビルドスクリプト由来のものは `OUT_DIR`（ターゲットディレクトリ + プロファイル +
  ビルドハッシュ）を含むため
- バンドルに無いアセットをレンダリングすると**パニックする**。
  これは「ビルド/デプロイの不整合」を意味するので、握りつぶさないこと

## Tailwind 連携

- `build.rs` から `topcoat::tailwind::BuildConfig::new().render()` を呼ぶ
- Node も PostCSS も Vite も使わない。**Tailwind 標準 CLI（既定 4.3.2）を
  GitHub からダウンロード**して実行する（`target/topcoat/cache/tailwind` にキャッシュ）
- 出力は `$OUT_DIR/tailwind.css`。`tailwind::stylesheet!()` で参照すると
  普通の Topcoat アセットとして扱われる
- `dependencies` と `build-dependencies` の**両方**で `tailwind` feature を有効にする
  （build 側は `default-features = false`）
- カスタム CSS を使うなら `.input("src/styles/global.css")`

### 🔴 ハマりどころ

- **クラス走査は `.gitignore` に従う。** `/target` の行が無いとビルド成果物まで走査して、
  遅いうえに前のビルドのクラスが復活する。無い場合は `.cwd("src")` で範囲を絞る
- **クラス名を実行時に組み立てると Tailwind から見えない。** 静的な文字列で書くこと
- オフラインビルドが必要なら `.executable("tailwindcss")` か
  `.executable_env("TAILWIND_CLI")` で既存の CLI を使う（ダウンロードが走らなくなる）。
  `.version_checksum(...)` でダウンロード物の sha256 を検証することもできる
- `render()` は `rerun-if-*` を出さないので、Cargo の既定（パッケージ内の
  非 ignore ファイルが変わったら再実行）が効く。自分で `rerun-if-*` を書くと**上書きされる**
- **`target/` を含むディレクトリを rerun 対象に指定しないこと。** Cargo の
  ディレクトリ走査は `.gitignore` を無視するので、自分の出力で再実行が無限に走る

## フォント（`font` feature）

このプロジェクトの「Geist をバンドルする」決定に直接効く。

- `font!` — `@font-face` ブロックを自分で書いて宣言する
- `fontsource_font!` — [Fontsource] のカタログから宣言する。
  ファミリ名・ウェイト・スタイル・サブセットを**コンパイル時に検証**してくれる
- `weight` / `style` / `subset` 引数で絞り込む。組み合わせごとに別ファイルなので
  使う分だけ指定する
- **`host: Asset` を付けるとビルド時にダウンロードして自分のオリジンから配信する**
  （コンテンツハッシュ付きの Topcoat アセットになる）。既定は jsDelivr CDN
- `.discover()` が `font!` 宣言を自動で拾う。手動なら `.font(CONST)`
- `<head>` には `topcoat::font::link(font: CONST)` を置く

### Geist はカタログにある

`topcoat-font-0.8.1/fonts.json` を確認した。

| id | family | weights | subsets | license |
| --- | --- | --- | --- | --- |
| `geist` | Geist | 100–600（可変） | latin / latin-ext / cyrillic ほか | OFL-1.1 |
| `geist-mono` | Geist Mono | 100–600（可変） | latin / latin-ext / symbols2 ほか | OFL-1.1 |

`design.md` が要求する Geist Mono 500 と Geist 600 は両方とも範囲内。
サブセット単位でファイルが分かれているので、`subset: Latin` を指定すれば
ラテンだけが落ちてくる。**woff2 を手で用意してサブセット化する必要は無い。**

## ランタイム（`runtime` feature）

**このポートフォリオでは使わない**（`design.md` のとおりクライアント状態がゼロのため）。
記録だけ残す。

- `$(...)` は**ランタイム式**。`expr!` マクロの糖衣。サーバーで 1 回評価して初期 HTML を作り、
  等価な JavaScript がページと一緒に配られてブラウザで再実行される
- 両言語で同じ挙動になる必要があるため、**Rust のごく一部の型とメソッドしか使えない**。
  脱出口として `raw!` がある
- `signal(cx, || 初期値)` でブラウザ側の状態を作る。0.8.0 で独自構文から
  普通の Rust 関数に変わった
- **サーバーがシグナルから読み戻した値はユーザー入力として扱う**（信頼しない）
- ほかに event handler / bind 属性 / `#[procedure]` / `#[shard]` がある
- ドキュメント自身が **"highly experimental and fairly limited today"** と書いている

## feature 一覧（0.8.1）

```
default = asset compression cookie discover font icon router runtime serve session view
その他 = alpine-ajax datastar font-fontsource htmx icon-iconify mail mail-smtp
         multipart sitemap sse tailwind tower ui websocket
full    = 全部
```

このプロジェクトで追加が要るのは **`tailwind`**、フォントを Fontsource から取るなら
**`font-fontsource`**、サイトマップを使うなら **`sitemap`**。

## 未実装 — 自分で埋める必要があるもの

このポートフォリオの移植に直接効くもの:

- [ ] **Markdown サポート** ← 作品データがこれ。`pulldown-cmark` などを自分で噛ませる
- [ ] **画像最適化・リサイズ** ← 旧 `astro:assets` の webp + srcset に相当するものが無い
- [ ] **静的エクスポート / プリレンダリング**
- [ ] `topcoat new`

0.5.0 の時点で未実装だったもののうち、**以下は実装された**:

- ✅ **サイトマップ** — `sitemap` feature（0.6.0 で追加）
- ✅ **ストリーミング SSR** — `live!` / `emit!` リージョン（0.7.0 で追加）
- ✅ **UI コンポーネント** — `topcoat ui` と `topcoat-ui` crate

## 参考リンク

- README（各機能のガイドへのリンク集）: https://github.com/tokio-rs/topcoat#learn-topcoat
- docs.rs（バージョン固定）: https://docs.rs/topcoat/0.8.1
- 公式サンプル: https://github.com/tokio-rs/topcoat/tree/main/examples
- Discord（tokio）: https://discord.gg/tokio

[Fontsource]: https://fontsource.org/
