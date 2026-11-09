# ドメインモデル

案件（Project）・技術（Technology）・作品（Work）のドメインを、関数型ドメインモデリングの形で書いたもの。
ドメインは**型**（データ）と**関数**（振る舞い）に分けて表す。型はふるまいを持たず、
ふるまいは型を受け取って型を返す関数として書く。

用語の定義は [`CONTEXT.md`](../CONTEXT.md)、型の関係を図にしたものは [`domain-model.html`](domain-model.html)、
データの流れは [`architecture.md`](architecture.md) を見ること。

## 記法

Scott Wlaschin『Domain Modeling Made Functional』の書き方に合わせる。

| 記法 | 意味 | Rust では |
| --- | --- | --- |
| `A AND B` | 両方を持つ（直積型） | `struct` |
| `A OR B` | どれか 1 つ（直和型） | `enum` |
| `A = Ulid` | 中身を包んだ単純型 | newtype（`struct A(Ulid)`） |
| `A -> B` | A を受け取り B を返す関数 | `fn(A) -> B` |
| `Result<A, E>` | 成功なら A、失敗なら E | `Result<A, E>` |

## データの持ち主

`Project` は、別システムから届く項目と、このサイトが決める項目を合わせ持つ集約。

| 項目 | 持ち主 | このサイトでは |
| --- | --- | --- |
| `id` / `title` / `description` / 種類 / `period` / `techStack` | 別システム | 読むだけ（写し） |
| 公開状態（`publication`） | **このサイト** | 書く・読む |
| 作品（`work`） | **このサイト** | 書く・読む |
| 技術 | 別システム | 読むだけ（写し） |

- 案件と技術は、実務も個人開発も別システムから届く
- 同期は別システムのイベントをきっかけにする。具体的な仕組みは未定
- 同期のときに案件と技術の写しを保存し、表示は写しから組み立てる
- 同期は、別システムから届く項目だけを書き換える。公開状態と作品には触れない
- **同期は案件を消さない。** 別システムから消えた案件は、作品ごと残して「別システムから消えた」印を付ける。
  作品はこのサイトで作った多くのデータを持つので、消すと取り戻せないため
- このサイトで案件を消せるのは、別システムから消えた案件だけ。消すと作品も一緒に消える
- v1 は同期も DB も無く、データをコードに直接書く

## 型

### 単純型

```
data ProjectId    = Ulid
data TechnologyId = Ulid
```

中身はどちらも ULID だが、別の型なので取り違えられない。ID は別システムが発行し、このサイトは作らない。

### 期間

```
data Period =
    Past
        start: NaiveDate
        AND end: NaiveDate
    OR Ongoing
        start: NaiveDate
```

- 終了日の無い `Past` と、終了日のある `Ongoing` は書けない
- 実務の案件も個人開発も期間を持つ。どちらも開始日は必ずあり、終了日は無いこともある

### 案件の種類と作品

```
kind ProjectKind =
    Professional
    OR Personal

data Professional =
    （データなし）

data Personal =
    work: Option<Work>

data Work =
    ...
```

- `ProjectKind` は、案件の種類を表す**型の集まり**。`Professional` と `Personal` の 2 つの型だけが属し、ほかの型は足せない
  （コードでは封じたトレイト。下の「案件」を参照）
- 期間の形は種類によらず同じなので、**期間から種類を推測しない**。種類は別システムのデータが明示する
- 作品は個人開発の中に持つ。実務の案件は作品を持てず、1 つの個人開発に作品は 1 つまで。どちらも型で守る
- 作品は技術スタックを持たない。持ち主の個人開発の技術スタックをすべてそのまま見せる
- 作品の本文・画像・スラッグ・並び順などの項目は未設計

### 公開状態

```
data Publication =
    Published
    OR Unpublished
    OR RemovedUpstream
```

- 公開している案件だけが、年表・使用件数・作品として見える
- `RemovedUpstream` は、別システムから消えた案件。最初から無かったものとして扱い、二度と公開しない
- 「消えたのに公開している」状態は、2 つの真偽値ではなく 1 つの直和型にすることで書けなくしている

### 案件

```
data Project<K: ProjectKind> =
    id: ProjectId
    AND title: String
    AND description: String
    AND period: Period
    AND techStack: Set<TechnologyId>
    AND publication: Publication
    AND kind: K

data AnyProject =
    Professional of Project<Professional>
    OR Personal of Project<Personal>
```

- `Project` は種類 `K` を型の引数に持つ。`Project<Professional>` と `Project<Personal>` は別の型になり、
  個人開発だけの関数は `Project<Personal>` だけが受け取る
- `K` に使えるのは `ProjectKind` に属する 2 つの型だけ。コードでは `K: ProjectKind` という制約（トレイト境界）で守り、
  `ProjectKind` は封じたトレイト（sealed trait）にして、ほかの型が実装できないようにする
- `AnyProject` は、種類がまだ分からない案件。保存先から読んだ案件や、種類の混ざった一覧（年表など）に使う。
  種類ごとの関数を呼ぶ前に、`AnyProject` を 1 度だけ振り分ける
- `techStack` は技術そのものではなく、ID の集合で参照する。同じ技術は 2 回入らず、順序は持たない

### 技術とカテゴリ

```
data Category =
    Language
    OR Framework
    OR Database
    OR Infrastructure
    OR BusinessTool

data Technology =
    id: TechnologyId
    AND name: String
    AND category: Category
```

| `Category` | 表示名 |
| --- | --- |
| `Language` | 言語 |
| `Framework` | ライブラリ・フレームワーク |
| `Database` | DB |
| `Infrastructure` | インフラ・開発ツール |
| `BusinessTool` | 業務ツール |

技術は必ずどれか 1 つのカテゴリに属する。カテゴリは技術の側が持ち、案件は持たない。

### エラー

```
data PeriodError =
    EndBeforeStart

data PublishError =
    RemovedUpstream

data DeleteError =
    NotRemovedUpstream

data AddWorkError =
    RemovedUpstream
    OR AlreadyHasWork
```

## 型どうしの関係

```
AnyProject ──┬── Project<Professional>
             └── Project<Personal>

Project<K> ──┬── ProjectId
             ├── Period
             ├── Publication
             ├── K（Professional か Personal）── Work（Personal のみ、1 つまで）
             └── Set<TechnologyId> ┄┄┄> Technology ──┬── TechnologyId
                                                     └── Category
```

- 実線（`──`）は値として中に持つ
- 点線（`┄┄┄>`）は ID で参照する。`Project` と `Technology` は別の集約で、片方を変えても、もう片方の中身は変わらない
- 案件と技術は多対多。1 つの案件は複数の技術を使い、1 つの技術は複数の案件で使われる

## 関数

### 生成（スマートコンストラクタ）

外から届いた形を、ドメインの型に変換する。失敗しうるものは `Result` を返す。

```
periodFromDates    : NaiveDate -> Option<NaiveDate> -> Result<Period, PeriodError>
projectIdFrom      : Ulid -> ProjectId
technologyIdFrom   : Ulid -> TechnologyId
```

- `periodFromDates` は、終了日が無ければ `Ongoing`、あれば `Past` を返す。
  終了日が開始日より前なら `EndBeforeStart` で失敗する
- 種類は別システムのデータをそのまま使うので、期間から種類を導く関数は持たない
- コードでは `Period::from_dates`、`From<Ulid> for ProjectId`、`From<Ulid> for TechnologyId`

### 案件の状態を変える（このサイトが持つ項目だけ）

```
publish            : Project<K> -> Result<Project<K>, PublishError>
unpublish          : Project<K> -> Project<K>
markRemovedUpstream: Project<K> -> Project<K>
ensureDeletable    : Project<K> -> Result<(), DeleteError>
addWork            : Work -> Project<Personal> -> Result<Project<Personal>, AddWorkError>
```

- 公開状態の関数は、どの種類 `K` にも 1 度だけ書く（コードでは `impl<K: ProjectKind> Project<K>`）
- `addWork` は `Project<Personal>` だけが受け取る（コードでは `impl Project<Personal>`）。
  実務の案件に作品を足す呼び出しは、型の上で書けない

- `publish` は、別システムから消えた案件なら `RemovedUpstream` で失敗する
- `unpublish` は、公開している案件を非公開にする。消えた案件はそのまま
- `markRemovedUpstream` は、同期が別システムから消えた案件に使う。公開状態は `RemovedUpstream` になる
- `addWork` は、別システムから消えた案件なら `RemovedUpstream`、
  すでに作品があれば `AlreadyHasWork` で失敗する（この順に検査する）。
  公開していない個人開発にも足せる。作品は、個人開発を公開するまで見えない
- `ensureDeletable` は、別システムから消えた案件でなければ `NotRemovedUpstream` で失敗する。
  実際に消すのは保存を担う側で、消す前に必ずこれを通す
- どれも純粋な関数。コードでは `self` を受け取り、新しい `Project` を返すメソッドにする（`&mut self` で書き換えない）。
  失敗すると渡した値は返らないが、失敗したものは保存し直さないので困らない。
  `ensureDeletable` は何も変えないので `&self` を受け取る

### 表示のための計算（未実装）

どれも純粋な関数で、入力だけから出力が決まる。結果は保存しない。

```
stackPanel : List<AnyProject> -> List<Technology> -> List<StackRow>
timeline   : List<AnyProject> -> List<TimelineItem>
```

- どちらも公開している案件だけを対象にする。作品も、持ち主の個人開発が公開しているときだけ見せる
- `stackPanel` は案件だけを数え、作品は数えない。どの公開している案件にも使われていない技術は行にしない
- 件数とゲージは、保存した写しから表示のたびに計算する。保存すると元のデータと食い違う余地が生まれ、
  ゲージは最大値が変わるたびに全行が変わるため

`StackRow` と `TimelineItem` は表示用の型で、ドメインの外（読み取り側）に置く。
件数・ゲージ・並び順の規則は [`architecture.md`](architecture.md) の「使用技術（Stack パネル）」にある。

## 不変条件のありか

| 規則 | 守り方 |
| --- | --- |
| 継続中の期間は終了日を持たない | 型（`Ongoing` に `end` が無い） |
| 1 つの案件に同じ技術は 2 回入らない | 型（`Set`） |
| 案件 ID と技術 ID を取り違えない | 型（別の単純型） |
| 案件の種類を期間から取り違えない | 型（種類を型の引数 `K` で明示して持つ） |
| 案件の種類は実務の案件と個人開発の 2 つだけ | 型（封じたトレイト `ProjectKind`） |
| 実務の案件は作品を持たない | 型（`Professional` にデータが無く、`addWork` は `Project<Personal>` だけが受け取る） |
| 1 つの個人開発に作品は 1 つまで | 型（`Option<Work>`） |
| 別システムから消えた案件は公開しない | 型（`Publication` の 1 つの状態）と関数（`publish`） |
| 終了日は開始日より前にならない | 関数（`periodFromDates`）。`Period::Past { .. }` と直接書くと検査を通らない |
| 消せるのは別システムから消えた案件だけ | 関数（`ensureDeletable`）。保存を担う側が消す前に呼ぶ |
| 別システムから消えた案件には作品を足せない | 関数（`addWork`） |
| 技術の名前が重複しない | 未決。1 つの `Technology` では検査できない、一覧全体の規則 |

## 未決

- 同期で新しく届いた案件の公開状態の既定値
- 別システムから消えた案件が、また届いたときの扱い
