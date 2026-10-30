# ドメインモデル

案件（Project）と技術（Technology）のドメインを、関数型ドメインモデリングの形で書いたもの。
ドメインは**型**（データ）と**関数**（振る舞い）に分けて表す。型はふるまいを持たず、
ふるまいは型を受け取って型を返す関数として書く。

決定の経緯と未決事項は [`domain-model.html`](domain-model.html)、
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

## 型

### 単純型

```
data ProjectId    = Ulid
data TechnologyId = Ulid
```

中身はどちらも ULID だが、別の型なので取り違えられない。ID は別アプリが発行し、このサイトは作らない。

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
        period: Period
    OR Personal
```

- 終了日の無い `Past` と、終了日のある `Ongoing` は書けない
- 個人開発は期間を持たない。`Personal` にはデータが無い

### 案件

```
data Project =
    id: ProjectId
    AND title: String
    AND description: String
    AND kind: ProjectKind
    AND techStack: Set<TechnologyId>
    AND showInTimeline: bool
```

- `techStack` は技術そのものではなく、ID の集合で参照する。同じ技術は 2 回入らず、順序は持たない
- `id` と `showInTimeline` はまだコードに無い。`showInTimeline` の名前は仮

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

カテゴリは技術の側が持つ。案件はカテゴリを持たない。

### エラー

```
data PeriodError =
    EndBeforeStart
```

## 型どうしの関係

```
Project ──┬── ProjectId
          ├── ProjectKind ── Period
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
projectKindFrom    : Option<Period> -> ProjectKind
technologyIdFrom   : Ulid -> TechnologyId
```

- `periodFromDates` は、終了日が無ければ `Ongoing`、あれば `Past` を返す。
  終了日が開始日より前なら `EndBeforeStart` で失敗する
- `projectKindFrom` は、期間が無ければ `Personal` を返す。失敗しない
- コードでは `Period::from_dates`、`ProjectKind::from_period`、`From<Ulid> for TechnologyId`

### 表示のための計算（未実装）

どれも純粋な関数で、入力だけから出力が決まる。保存はしない。

```
stackPanel : List<Project> -> List<Technology> -> List<StackRow>
timeline   : List<Project> -> List<TimelineItem>
```

`StackRow` と `TimelineItem` は表示用の型で、ドメインの外（読み取り側）に置く。
件数・ゲージ・並び順の規則は [`architecture.md`](architecture.md) の「使用技術（Stack パネル）」にある。

## 不変条件のありか

| 規則 | 守り方 |
| --- | --- |
| 個人開発は期間を持たない | 型（`Personal` にデータが無い） |
| 継続中の期間は終了日を持たない | 型（`Ongoing` に `end` が無い） |
| 1 つの案件に同じ技術は 2 回入らない | 型（`Set`） |
| 案件 ID と技術 ID を取り違えない | 型（別の単純型） |
| 終了日は開始日より前にならない | 関数（`periodFromDates`）。`Period::Past { .. }` と直接書くと検査を通らない |
| 技術の名前が重複しない | 未決。1 つの `Technology` では検査できない、一覧全体の規則 |

## v2: Work — 作品

`Project` とは別の概念で、このサイトが書き込む唯一の集約。型と関数はまだ設計していない。
決まっている規則は、スラッグが重複しないことと、並び順が一意であること。
状態（下書き・公開など）を持つなら、状態ごとの直和型と、状態を移す関数で表す。
