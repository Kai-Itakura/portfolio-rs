# 構成（決定事項）

移植を始める前に決めた最終構成。**実装前のスペックであり、実装そのものではない。**
ここに書かれた構造に沿って、コードはオーナーが自分で書く。

## 全体像

| | 移行元 | 移行先 |
| --- | --- | --- |
| フレームワーク | Astro 5（完全静的出力） | Topcoat 0.8.1（サーバーレンダリング） |
| 言語 | TypeScript / Astro | Rust |
| スタイル | Tailwind CSS v4（Node 経由） | Tailwind CSS v4（Topcoat 経由・Node 不要） |
| 作品データ | Content Collections（Markdown） | 未定（下記「作品データ」参照） |
| 画像 | `astro:assets`（webp + srcset 自動生成） | Topcoat の asset システム（最適化は自前） |
| フォント | Google Fonts（CDN） | Topcoat の `font-fontsource` で自己ホスト |
| ホスティング | Cloudflare Workers（静的アセット） | Google Cloud Run（コンテナ） |

Cloud Run を選んだ理由: Topcoat は `#[tokio::main]` の常駐ランタイムを要求するため、
エッジ/サーバーレス関数では動かず、コンテナが必要。

## ディレクトリ構成

```
portfolio-rs/
├── Cargo.toml
├── build.rs                  # Tailwind をビルド時に実行する
├── rust-toolchain.toml
├── Dockerfile                # Cloud Run 向け multi-stage
├── .gitignore                # /target の行は必須（下記の理由参照）
├── .dockerignore             # .gitignore を除外しないこと
├── .github/workflows/ci.yml
├── docs/
├── src/
│   ├── main.rs               # topcoat::start(app::router())
│   ├── app.rs                # ルートレイアウト + トップページ + router()
│   ├── app/
│   │   ├── about.rs
│   │   ├── works.rs
│   │   └── works/
│   │       └── slug.rs
│   ├── components/           # 共通コンポーネント
│   └── styles/
│       └── global.css        # 旧 src/styles/global.css をそのまま移植
├── assets/                   # 旧 src/assets/ の画像
└── content/                  # 作品データ
```

## ルーティング

`module_router!()` によるモジュール構成ベースのルーティングを使う。
Astro のファイルベースルーティングとほぼ同じ考え方になる。

| URL | 移行先ファイル | 移行元 |
| --- | --- | --- |
| `/` | `src/app.rs` | `src/pages/index.astro` |
| `/about` | `src/app/about.rs` | `src/pages/about.astro` |
| `/works` | `src/app/works.rs` | `src/pages/works/index.astro` |
| `/works/{slug}` | `src/app/works/slug.rs` | `src/pages/works/[slug].astro` |

`src/app.rs` のレイアウトが全ページを包む（旧 `BaseLayout.astro` に相当）。

動的セグメントは、`works/slug.rs` の中で `#[path_param]` を宣言すると
`slug` モジュール自体が `{slug}` になる。URL に placeholder を書く必要はない。

## 移植するコンポーネント

旧リポジトリの 18 コンポーネントを F3 デザインに対応づけたもの。
見た目の仕様は [`design.md`](design.md) を見ること。

新デザインにクライアントサイドの状態は無い。ハンバーガーメニューも含めてすべて CSS で組むので、
Topcoat の `$(...)` 式も Islands も使わない。

| 旧コンポーネント | 移行後 |
| --- | --- |
| `BaseLayout.astro` | `src/app.rs` の `#[layout]` |
| `Header.astro` / `Nav.astro` | 共通コンポーネントのナビ。860px 以下でドロワー。開閉は `<input type="checkbox">` + `:has()` |
| `Footer.astro` | 共通コンポーネントのフッター |
| `Hero.astro` | 各ルートのヒーロー。トップは `$ whoami` の 4 行 |
| `Container.astro` | 不要。左右余白はトークンで持つ |
| `Profile.astro` | `/about` のプロフィール 2 カラム |
| `Posts.astro` | 共通コンポーネントのカード |
| `Hobby.astro` / `HobbyList.astro` | カードを使い回す |
| `Skills.astro` / `Bar.astro` | `/about` の年表 + 使用件数 |
| `WorksHeader.astro` / `WorksBody.astro` / `WorksImage.astro` | `/works/{slug}` の帯画像・本文 + メタ・スクショ 2 枚 |
| `Pagination.astro` | `/works/{slug}` の前後ナビ |
| `Button.astro` | 共通コンポーネントのボタン（ソリッド / ゴースト） |
| `DefinitionList.astro` | `/works/{slug}` の `.metadl` |
| `SnsLinks.astro` | FontAwesome の SVG |

### 前後の作品ナビゲーションの仕様

`order` の昇順に並べ、**`prev` が前の作品、`next` が次の作品**。
端では片方が空になり、そちらのリンクは表示しない。

Astro では `getStaticPaths` 内で計算していたが、Topcoat には静的生成が無いので
リクエスト時に解決する。

## 作品データ

旧構成は `src/content/works/*.md`（Content Collections）:

- ファイル名（id）がそのまま URL スラッグ
- `order` フィールドで表示順を制御
- Markdown 本文が作品詳細ページの about 欄になる
- フロントマター: `title` / `order` / `tools[]` / `time` / `url` / `lang` /
  `topImage` / `heroImage` / `mockUpImage` / `image1` / `image2`

**Topcoat には Markdown サポートが無い**（ロードマップ段階）ので、以下のいずれかを選ぶ:

1. Markdown を維持し、`pulldown-cmark` などでパースする（旧データをそのまま使える）
2. データを Rust の構造体（または TOML / JSON）に持ち替える（型が付く。CMS 的な編集性は落ちる）

起動時に一度読んでメモリに載せる想定。作品数が少ないのでDBは不要。

## メタ情報（移行時に失ってはいけないもの）

旧 `BaseLayout.astro` / `src/lib/constants.ts` にあるもの:

- `<title>` — `pageTitle ? "{pageTitle} | Kai Itakura" : "Kai Itakura"`
- `description` / OG（`og:title` `og:description` `og:url` `og:site_name`
  `og:type=website` `og:locale=ja_JP` `og:image` + width/height）
- `twitter:card = summary_large_image`
- `canonical`
- 🔴 **`google-site-verification`** — Search Console の所有権確認。消すと確認が外れる
- `favicon` / `apple-touch-icon`
- サイトマップ（`<link rel="sitemap">`）。Topcoat の `sitemap` feature を使う
- Google Analytics（`PUBLIC_GA_ID` が設定されているときだけ出力する）

`site` URL（OG / canonical / sitemap の絶対 URL 用）は
現在 `https://portfolio.itakai199969-e42.workers.dev`。Cloud Run 移行後に差し替える。

## スタイリング

- トークン（色 9 / フォント 2 / 角丸 3 / ブレークポイント 2）は [`design.md`](design.md) にある。
  `@theme` の中身はあの表がそのまま入る
- ブレークポイントは **860px / 620px**。`@theme` で `--breakpoint-md` / `--breakpoint-sm` を
  上書きして、記述は `max-md:` / `max-sm:` のまま使う
- レスポンシブは **`@media` のみ**。`container-type` はどこにも付けない
- 旧 `src/styles/global.css` から引き継ぐのは `@layer base` のベーススタイルだけ。
  `@theme` のトークンは `design.md` のものに置き換える
- 引き継ぐベーススタイルに `scroll-behavior: smooth` が含まれる。
  `prefers-reduced-motion: reduce` で抑制すること

## デプロイ（Cloud Run）

学習対象外。オーナーが依頼したときだけ Claude が用意してよい。

要件:

- multi-stage build。ビルドステージで `cargo build --release` +
  `topcoat asset bundle --release`
- **ランタイムステージにはバイナリと `assets/` を同じディレクトリに置く。**
  `AssetBundle::load()` は実行ファイルの隣の `assets/` を読む
- **`ENV HOST=0.0.0.0`** が必須。既定は `127.0.0.1` なので、これが無いと
  Cloud Run からの接続を受けられない。`PORT` は Cloud Run が注入する（8080）
- `topcoat-cli` はアセットバンドルに必要。ビルドが重いので別ステージに分けてキャッシュする。
  **ライブラリと同じバージョンを入れること**（ずれると CLI が警告を出す）
- **ビルド時にネットワークが要る**（`build.rs` が Tailwind CLI を GitHub から取得する）。
  オフラインビルドが必要なら `BuildConfig::executable_env("TAILWIND_CLI")` を使う
- `topcoat::start` は SIGTERM でグレースフルシャットダウンする（Cloud Run と相性が良い）
- コスト対策: **`min-instances=0`（scale-to-zero）にすること。**
  これを 1 以上にすると常時課金になる
