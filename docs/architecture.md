# 構成（決定事項）

移植を始める前に決めた最終構成。**実装前のスペックであり、実装そのものではない。**
ここに書かれた構造に沿って、コードはオーナーが自分で書く。

## 全体像

| | 移行元 | 移行先 |
| --- | --- | --- |
| フレームワーク | Astro 5（完全静的出力） | Topcoat 0.9.0（サーバーレンダリング） |
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

## 使用技術（Stack パネル）

トップと `/about` の「使用件数」のパネル。**件数はハードコードせず、案件ごとの使用技術から
コードで数える。** 最終的には、転職活動用に別で管理しているアプリ（プロジェクト・技術スタック・
STAR ログ）と連携して出す。v1 の案件データは、その置き換えを見越した形にしておく。

### カテゴリ

別システムと同じ 5 カテゴリで持つ。カテゴリは固定。

| カテゴリ | パネル | 今のデータで入るもの |
| --- | --- | --- |
| 言語 | 表示する | TypeScript, Sass, Go, Node.js, CSS, HTML, JavaScript, Rust, SQL |
| ライブラリ・フレームワーク | 表示する | React, Next.js, Tailwind CSS, React Router v7, Topcoat, NestJS, Prisma |
| DB | 表示する | MySQL, PostgreSQL |
| インフラ・開発ツール | 表示する | Docker, AWS, GitHub Actions, Kubernetes |
| 業務ツール | 表示しない | （v1 ではデータにも持たない） |

分類で迷ったものの扱い: HTML / CSS / Sass / SQL / Node.js は言語、Tailwind CSS と Prisma は
ライブラリ・フレームワーク。別システムに出した指示ではカテゴリ名が「フレームワーク」なので、
連携するときに名前をそろえる。

### 数える対象（v1）

- **数えるもの**：上の表示する 4 カテゴリに入るもの
- **数えないもの**：業務ツール（Jira / Slack / Figjam / Backlog / Confluence）、デザインツール、
  AI ツール、Git クライアントとホスティング、ホスティング・SaaS・EC 基盤、OS。
  加えて、個別に外したもの（Smarty / Python / PowerShell / PHP / Symfony / Twig / Fiber）
- **表記**：`HTML` / `CSS`、`React Router v7`、`GitHub Actions`。
  AWS のサービス名（CloudFront、Aurora など）は `AWS` にまとめる
- 元データは職務経歴書の技術欄。業務内容の本文にだけ出てくる技術は、個別に判断して足す（React）か外す（jQuery）

業務ツール以外で外したものや、本文から足し引きしたものを連携後にどう再現するかは、
システム連携ができた時点で決める。

### 案件（2026-09-30 時点・10 件）

| # | 期間 | 案件 | 技術 |
| --- | --- | --- | --- |
| 0 | 22/06〜22/12 | デジハリ（卒業制作ほか） | JavaScript, HTML, Sass |
| 1 | 23/01〜23/06 | 自己学習・ポートフォリオ | TypeScript, Next.js, Sass, React |
| 2 | 23/07〜23/07 | エンタメ Web サイト運用 | TypeScript, Next.js, Sass, CSS, AWS, GitHub Actions |
| 3 | 23/08〜23/10 | 食品 toC EC 改修 | JavaScript, HTML, CSS |
| 4 | 23/11〜25/02 | デジタル教科書 新機能開発 | TypeScript, Node.js |
| 5 | 25/03〜25/09 | toB EC リプレイス | TypeScript, Node.js, Tailwind CSS, AWS, Docker, MySQL |
| 6 | 25/08〜25/09 | 自社サイトリプレイス | TypeScript, Sass, React, React Router v7, Docker |
| 7 | 26/01〜26/06 | 工場生産管理システム | Go, TypeScript, React, React Router v7, Docker, Kubernetes, AWS, GitHub Actions |
| x | 期間なし | 個人開発 | TypeScript, Go, React, Next.js, NestJS, Docker, Prisma, AWS, SQL, PostgreSQL |
| 9 | 26/07〜現在 | このサイト | Rust, Topcoat, Tailwind CSS |

このサイトの Cloud Run は、デプロイした時点で足す。

### 並び順

カテゴリの中で、次の順に比べる。

1. 件数が多い順
2. 最後に使った時期（案件の終了月）が新しい順
3. 使い始めた時期（案件の開始月）が新しい順
4. 名前順

**個人開発は 2 と 3 で最も古い扱いにする。** 件数には含めるが、同じ件数どうしの比較では
実務とこのサイトを優先する。3 は「最近身に付けた技術ほど前に出す」ための基準で、
これが無いと 26/06 で並ぶ Go が名前順で後ろに回る。

### 表示

- パネルの中で 4 カテゴリを**縦に 1 列**に積み、各カテゴリの中は**2 列**で並べる。860px 以下ではカテゴリの中も 1 列
- **トップは各カテゴリ上位 4 件まで**、`/about` は全件。2 列で並べるので、トップは偶数件にして行を埋める。
  トップは見出しの右に「4 / 9」のように全体の何件中何件かを出し、`/about` に続きがあることを示す
- ゲージは 7 マスで全カテゴリ共通。カテゴリごとに最大値を変えると、DB の 1 件が満タンに見えるため

上の案件データでの結果は次のとおり（太字がトップに出るもの）。

| カテゴリ | 並び（件数） |
| --- | --- |
| 言語 | **TypeScript 7 → Sass 4 → Go 2 → Node.js 2** → CSS 2 → HTML 2 → JavaScript 2 → Rust 1 → SQL 1 |
| ライブラリ・フレームワーク | **React 4 → Next.js 3 → Tailwind CSS 2 → React Router v7 2** → Topcoat 1 → NestJS 1 → Prisma 1 |
| DB | **MySQL 1 → PostgreSQL 1** |
| インフラ・開発ツール | **Docker 4 → AWS 4 → GitHub Actions 2 → Kubernetes 1** |

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
- **ビルド時にネットワークが要る**（`build.rs` が Tailwind CLI を GitHub から、
  Lucide のアイコンセットを Iconify から取得する）。
  オフラインビルドが必要なら、Tailwind は `BuildConfig::executable_env("TAILWIND_CLI")`、
  アイコンは `iconify::BuildConfig::cache_dir(...)` でキャッシュをリポジトリに置く
- `topcoat::start` は SIGTERM でグレースフルシャットダウンする（Cloud Run と相性が良い）
- コスト対策: **`min-instances=0`（scale-to-zero）にすること。**
  これを 1 以上にすると常時課金になる
