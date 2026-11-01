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

| | 案件・技術 | 作品・公開する案件 |
| --- | --- | --- |
| 正のデータ | 別システム | **このサイト** |
| このサイトでは | 読むだけ（写し） | 書く・読む |

- 案件と技術は、実務も個人開発も別システムから届く。このサイトは書き込まない
- 同期は別システムのイベントをきっかけにする。具体的な仕組みは未定
- 同期のときに案件と技術の写しを保存し、表示は写しから組み立てる
- v1 は同期も DB も無く、データをコードに直接書く

## 型

### 単純型

```
data ProjectId    = Ulid
data TechnologyId = Ulid
```

中身はどちらも ULID だが、別の型なので取り違えられない。ID は別システムが発行し、このサイトは作らない。

### 期間と案件の種類

```
data Period =
    Past
        start: NaiveDate
        AND end: NaiveDate
    OR Ongoing
        start: NaiveDate

data ProjectKind =
    Professional
    OR Personal
```

- 終了日の無い `Past` と、終了日のある `Ongoing` は書けない
- 実務の案件も個人開発も期間を持つ。どちらも開始日は必ずあり、終了日は無いこともある
- 期間の形が同じなので、**期間から種類を推測しない**。種類は別システムのデータが明示する
- `ProjectKind` はデータを持たない判別子になる

### 案件

```
data Project =
    id: ProjectId
    AND title: String
    AND description: String
    AND kind: ProjectKind
    AND period: Period
    AND techStack: Set<TechnologyId>
```

- `techStack` は技術そのものではなく、ID の集合で参照する。同じ技術は 2 回入らず、順序は持たない
- 公開するかどうかは案件に持たせない。別システムは、このサイトの見せ方を知らない

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
カテゴリの一覧は見直す予定（未決）。

### 公開する案件（このサイトが持つ）

```
data PublishedProjects = Set<ProjectId>
```

- このサイトで見せる案件の ID の集合。公開しない案件は、年表にも使用件数にも出ない
- 名前は仮

### 作品（このサイトが持つ）

```
data Work =
    project: ProjectId
    AND ...
```

- 作品は案件とは別の集約で、個人開発を ID で参照する
- 1 つの個人開発に作品は 1 つだけ。実務の案件は作品を持たない
- 作品は技術スタックを持たない。参照する個人開発の技術スタックをすべてそのまま見せる
- 作品の作成は、公開する個人開発を選ぶところから始まる
- 参照する個人開発が公開されていなければ、作品も見せない
- 本文・画像・スラッグ・並び順などの残りの項目は未設計

### エラー

```
data PeriodError =
    EndBeforeStart
```

## 型どうしの関係

```
Project ──┬── ProjectId
          ├── ProjectKind
          ├── Period
          └── Set<TechnologyId> ┄┄┄> Technology ──┬── TechnologyId
                                                  └── Category

Work ┄┄┄> Project（個人開発のみ）

PublishedProjects ┄┄┄> Project
```

- 実線（`──`）は値として中に持つ
- 点線（`┄┄┄>`）は ID で参照する。参照し合う型どうしは別の集約で、片方を変えても、もう片方の中身は変わらない
- 案件と技術は多対多。1 つの案件は複数の技術を使い、1 つの技術は複数の案件で使われる
- 作品と個人開発は 1 対 1。個人開発は作品を持たないこともある

## 関数

### 生成（スマートコンストラクタ）

外から届いた形を、ドメインの型に変換する。失敗しうるものは `Result` を返す。

```
periodFromDates    : NaiveDate -> Option<NaiveDate> -> Result<Period, PeriodError>
technologyIdFrom   : Ulid -> TechnologyId
```

- `periodFromDates` は、終了日が無ければ `Ongoing`、あれば `Past` を返す。
  終了日が開始日より前なら `EndBeforeStart` で失敗する
- 種類は別システムのデータをそのまま使うので、期間から種類を導く関数は持たない
- コードでは `Period::from_dates`、`From<Ulid> for TechnologyId`

### 表示のための計算（未実装）

どれも純粋な関数で、入力だけから出力が決まる。結果は保存しない。

```
stackPanel : List<Project> -> List<Technology> -> PublishedProjects -> List<StackRow>
timeline   : List<Project> -> PublishedProjects -> List<TimelineItem>
```

- どちらも公開する案件だけを対象にする
- `stackPanel` は案件だけを数え、作品は数えない。どの公開する案件にも使われていない技術は行にしない
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
| 案件の種類を期間から取り違えない | 型（`kind` を明示して持つ） |
| 終了日は開始日より前にならない | 関数（`periodFromDates`）。`Period::Past { .. }` と直接書くと検査を通らない |
| 作品は個人開発だけを参照する | 未実装。作品を作るときに検査する |
| 1 つの個人開発に作品は 1 つだけ | 未実装。作品の一覧全体の規則 |
| 技術の名前が重複しない | 未決。1 つの `Technology` では検査できない、一覧全体の規則 |
