# 構成（決定事項）

移植を始める前に決めた最終構成。**実装前のスペックであり、実装そのものではない。**
ここに書かれた構造に沿って、コードはオーナーが自分で書く。

## 全体像

| | 移行元 | 移行先 |
| --- | --- | --- |
| フレームワーク | Astro 5（完全静的出力） | Topcoat 0.5.0（サーバーレンダリング） |
| 言語 | TypeScript / Astro | Rust |
| スタイル | Tailwind CSS v4（Node 経由） | Tailwind CSS v4（Topcoat 経由・Node 不要） |
| 作品データ | Content Collections（Markdown） | 未定（下記「作品データ」参照） |
| 画像 | `astro:assets`（webp + srcset 自動生成） | Topcoat の asset システム（最適化は自前） |
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

旧リポジトリの 18 コンポーネント。素の `<script>` で書かれたインタラクションは、
Topcoat の `$(...)` 式（サーバーでもブラウザでも動く型付き Rust）に置き換えられる可能性がある。

| 旧コンポーネント | 備考 |
| --- | --- |
| `BaseLayout.astro` | → `src/app.rs` の `#[layout]` |
| `Header.astro` | スクロールでヘッダー背景。`scrollY > 450` のマジックナンバーは要見直し |
| `Nav.astro` | ハンバーガーメニュー。`data-open` + `group-data-[open=true]:` で状態管理 |
| `Footer.astro` | |
| `Hero.astro` | バブルアニメーション（`setInterval` で `.bubble` を生成） |
| `Container.astro` | |
| `Profile.astro` | |
| `Posts.astro` | |
| `Hobby.astro` / `HobbyList.astro` | |
| `Skills.astro` / `Bar.astro` | IntersectionObserver でスキルバーをカウントアップ |
| `WorksHeader.astro` / `WorksBody.astro` / `WorksImage.astro` | |
| `Pagination.astro` | 前後の作品ナビ |
| `Button.astro` | |
| `DefinitionList.astro` | |
| `SnsLinks.astro` | FontAwesome の SVG |

### 前後の作品ナビゲーションの仕様

旧サイトの仕様を踏襲して**意図的に逆順**にする（prev = 次のインデックス、next = 前のインデックス）。
端では空になり、リンクが非表示になる。

Astro では `getStaticPaths` 内で計算していたが、Topcoat には静的生成が無いので
リクエスト時に解決する形になる。

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
- サイトマップ（`<link rel="sitemap">`）。**Topcoat は生成してくれない**ので自前で作る
- Google Fonts（Kaisei Decol 700 / Pacifico / Roboto 500）
- Google Analytics（`PUBLIC_GA_ID` が設定されているときだけ出力する）

`site` URL（OG / canonical / sitemap の絶対 URL 用）は
現在 `https://portfolio.itakai199969-e42.workers.dev`。Cloud Run 移行後に差し替える。

## スタイリング

- 旧 `src/styles/global.css` を**そのまま持ってくる**。`@theme` のトークン
  （`--color-main` `--color-accent` `--font-pacifico` `--font-kaisei` `--font-roboto`）と
  `@layer base` のベーススタイルはそのまま使える
- モバイル（767px 以下）は `max-md:` プレフィックス
- 旧 SCSS のピクセル値は `text-[80px]` のような arbitrary value で移植済み

## デプロイ（Cloud Run）

学習対象外。オーナーが依頼したときだけ Claude が用意してよい。

要件:

- multi-stage build。ビルドステージで `cargo build --release` +
  `topcoat asset bundle --release`
- **ランタイムステージにはバイナリと `assets/` を同じディレクトリに置く。**
  `AssetBundle::load()` は実行ファイルの隣の `assets/` を読む
- **`ENV HOST=0.0.0.0`** が必須。既定は `127.0.0.1` なので、これが無いと
  Cloud Run からの接続を受けられない。`PORT` は Cloud Run が注入する（8080）
- `topcoat-cli` はアセットバンドルに必要。ビルドが重いので別ステージに分けてキャッシュする
- **ビルド時にネットワークが要る**（`build.rs` が Tailwind CLI を GitHub から取得する）。
  オフラインビルドが必要なら `BuildConfig::executable_env("TAILWIND_CLI")` を使う
- `topcoat::start` は SIGTERM でグレースフルシャットダウンする（Cloud Run と相性が良い）
- コスト対策: **`min-instances=0`（scale-to-zero）にすること。**
  これを 1 以上にすると常時課金になる

## 移行時に直すデザイン上の問題

旧サイトのデザインレビューで挙がった、移植のついでに直したいもの。

| | 内容 |
| --- | --- |
| 色トークンの散在 | `@theme` にあるのは `--color-main` と `--color-accent` だけで、実際の色は生ハードコードで散らばっている（`#F08774` / `cadetblue` / `#5BAE6D` / `#facf63` / `#F08275` / `#333` / `rgb(58 61 62 / 0.5)`）。移植時に `@theme` へ集約する |
| `prefers-reduced-motion` 未対応 | バブル・スキルバー・`scroll-behavior: smooth` が抑制設定を無視している |
| スキルバー | パーセンテージ（60〜80）に根拠が無く、JS が無いと空のまま埋まらない。表現方法ごと再考する |
| Hero | 100vh がほぼ単色 + 筆記体の名前だけで、情報量が無い |
| `WorksImage` の `h-[200vw]` | 高さがビューポート幅の 2 倍。画面幅に比例して縦に伸びるのは意図的か要確認 |
| `scrollY > 450` | ヘッダー背景の切り替え閾値がマジックナンバー。Hero の高さと連動していない |
