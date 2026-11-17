//! v1 のデータ（`data/v1.jsonc`）を読み、ドメインの型に変換する。
//!
//! ファイルはコンパイル時に `include_str!` で埋め込み、起動時に 1 度だけ読む。
//! ドメインの型は JSONC を知らないので、ここで受け取り用の型に読んでから、
//! ドメインの生成関数（`Period::from_dates` など）を通して変換する。

use std::collections::HashSet;
use std::fmt;

use chrono::NaiveDate;
use serde::Deserialize;
use ulid::Ulid;

use crate::domain::project::image_key::ImageKey;
use crate::domain::project::period::{Period, PeriodError};
use crate::domain::project::project_id::ProjectId;
use crate::domain::project::project_kind::{Personal, Professional};
use crate::domain::project::publication::Publication;
use crate::domain::project::work::Work;
use crate::domain::project::work_id::WorkId;
use crate::domain::project::{AnyProject, Project};
use crate::domain::technology::Technology;
use crate::domain::technology::category::Category;
use crate::domain::technology::technology_id::TechnologyId;

const V1_DATA: &str = include_str!("../../data/v1.jsonc");

/// 作品の本文（Markdown）。`include_str!` はリテラルのパスしか受け付けないので、
/// 作品 ID と `content/<作品 ID>.md` の対応をここに 1 行ずつ書く。
const WORK_BODIES: &[(&str, &str)] = &[];

/// 変換を終えた v1 のデータ。
pub struct V1Data {
    pub technologies: Vec<Technology>,
    pub projects: Vec<AnyProject>,
}

#[derive(Debug)]
pub enum LoadError {
    Parse(String),
    InvalidId(String),
    InvalidDate(String),
    InvalidPeriod { project: String, error: PeriodError },
    UnknownTechnology { project: String, technology: String },
    WorkOnProfessionalProject(String),
    MissingWorkBody(String),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(message) => write!(f, "cannot parse the file: {message}"),
            Self::InvalidId(id) => write!(f, "invalid ULID: {id}"),
            Self::InvalidDate(date) => write!(f, "invalid date (expected YYYY-MM-DD): {date}"),
            Self::InvalidPeriod { project, error } => {
                write!(f, "invalid period in project {project}: {error:?}")
            }
            Self::UnknownTechnology {
                project,
                technology,
            } => write!(f, "project {project} uses unknown technology {technology}"),
            Self::WorkOnProfessionalProject(project) => {
                write!(f, "professional project {project} must not have a work")
            }
            Self::MissingWorkBody(work) => {
                write!(
                    f,
                    "work {work} has no body in WORK_BODIES (content/{work}.md)"
                )
            }
        }
    }
}

impl std::error::Error for LoadError {}

pub fn load() -> Result<V1Data, LoadError> {
    parse(V1_DATA)
}

fn parse(text: &str) -> Result<V1Data, LoadError> {
    let file: FileData = jsonc_parser::parse_to_serde_value(text, &Default::default())
        .map_err(|error| LoadError::Parse(error.to_string()))?;

    let technologies = file
        .technologies
        .into_iter()
        .map(technology)
        .collect::<Result<Vec<_>, _>>()?;
    let known: HashSet<TechnologyId> = technologies
        .iter()
        .map(|technology| technology.id)
        .collect();
    let projects = file
        .projects
        .into_iter()
        .map(|data| project(data, &known))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(V1Data {
        technologies,
        projects,
    })
}

// ここから下は JSONC の形をそのまま受け取る型。ドメインの外にだけ置く。

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileData {
    technologies: Vec<TechnologyData>,
    projects: Vec<ProjectData>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TechnologyData {
    id: String,
    name: String,
    category: CategoryData,
}

#[derive(Deserialize)]
enum CategoryData {
    Language,
    Framework,
    Database,
    Infrastructure,
    BusinessTool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProjectData {
    id: String,
    kind: KindData,
    title: String,
    description: String,
    period: PeriodData,
    tech_stack: Vec<String>,
    publication: PublicationData,
    work: Option<WorkData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum KindData {
    Professional,
    Personal,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PeriodData {
    start: String,
    end: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum PublicationData {
    Published,
    Unpublished,
    RemovedUpstream,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WorkData {
    id: String,
    title: String,
    description: String,
    live_site: Option<String>,
    card_image: String,
    mock_up: String,
}

// ここから下は、受け取った値をドメインの型に変換する関数。

fn technology(data: TechnologyData) -> Result<Technology, LoadError> {
    Ok(Technology {
        id: TechnologyId::from(ulid(&data.id)?),
        name: data.name,
        category: match data.category {
            CategoryData::Language => Category::Language,
            CategoryData::Framework => Category::Framework,
            CategoryData::Database => Category::Database,
            CategoryData::Infrastructure => Category::Infrastructure,
            CategoryData::BusinessTool => Category::BusinessTool,
        },
    })
}

fn project(data: ProjectData, known: &HashSet<TechnologyId>) -> Result<AnyProject, LoadError> {
    let id = ProjectId::from(ulid(&data.id)?);
    let period = Period::from_dates(
        date(&data.period.start)?,
        data.period.end.as_deref().map(date).transpose()?,
    )
    .map_err(|error| LoadError::InvalidPeriod {
        project: data.id.clone(),
        error,
    })?;
    let tech_stack = data
        .tech_stack
        .iter()
        .map(|technology| {
            let technology_id = TechnologyId::from(ulid(technology)?);
            if known.contains(&technology_id) {
                Ok(technology_id)
            } else {
                Err(LoadError::UnknownTechnology {
                    project: data.id.clone(),
                    technology: technology.clone(),
                })
            }
        })
        .collect::<Result<HashSet<_>, _>>()?;
    let publication = match data.publication {
        PublicationData::Published => Publication::Published,
        PublicationData::Unpublished => Publication::Unpublished,
        PublicationData::RemovedUpstream => Publication::RemovedUpstream,
    };

    Ok(match data.kind {
        KindData::Professional => {
            if data.work.is_some() {
                return Err(LoadError::WorkOnProfessionalProject(data.id));
            }
            AnyProject::Professional(Project::new(
                id,
                data.title,
                data.description,
                period,
                tech_stack,
                publication,
                Professional,
            ))
        }
        KindData::Personal => {
            let work = data.work.map(work).transpose()?;
            AnyProject::Personal(Project::new(
                id,
                data.title,
                data.description,
                period,
                tech_stack,
                publication,
                Personal { work },
            ))
        }
    })
}

fn work(data: WorkData) -> Result<Work, LoadError> {
    let body = WORK_BODIES
        .iter()
        .find(|(id, _)| *id == data.id)
        .map(|(_, body)| *body)
        .ok_or_else(|| LoadError::MissingWorkBody(data.id.clone()))?;
    Ok(Work::new(
        WorkId::from(ulid(&data.id)?),
        data.title,
        data.description,
        data.live_site,
        body.to_string(),
        ImageKey::from(data.card_image),
        ImageKey::from(data.mock_up),
    ))
}

fn ulid(text: &str) -> Result<Ulid, LoadError> {
    Ulid::from_string(text).map_err(|_| LoadError::InvalidId(text.to_string()))
}

fn date(text: &str) -> Result<NaiveDate, LoadError> {
    text.parse()
        .map_err(|_| LoadError::InvalidDate(text.to_string()))
}
