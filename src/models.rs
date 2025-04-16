use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Project represents a software project or initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Repository represents a git repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: Uuid,
    pub url: String,
    pub local_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Branch represents a git branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub name: String,
    pub repository_id: Uuid,
    pub is_working_branch: bool,
    pub created_at: DateTime<Utc>,
}

/// PullRequest represents a pull/merge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: Uuid,
    pub number: String,
    pub title: String,
    pub repository_id: Uuid,
    pub source_branch_id: Uuid,
    pub target_branch_id: Uuid,
    pub status: PullRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ticket represents an issue or task in a tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub ticket_id: String,
    pub title: String,
    pub status: TicketStatus,
    pub ticket_type: TicketType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Dashboard represents a monitoring or visualization dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: Uuid,
    pub url: String,
    pub name: String,
    pub dashboard_type: DashboardType,
    pub created_at: DateTime<Utc>,
}

/// Notebook represents a data analysis notebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: Uuid,
    pub url: String,
    pub name: String,
    pub notebook_type: NotebookType,
    pub created_at: DateTime<Utc>,
}

/// CIPipeline represents a continuous integration/deployment pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIPipeline {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub pipeline_type: PipelineType,
    pub created_at: DateTime<Utc>,
}

/// Enums for various entity types

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PullRequestStatus {
    Open,
    Closed,
    Merged,
    Draft,
}

impl std::fmt::Display for PullRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullRequestStatus::Open => write!(f, "open"),
            PullRequestStatus::Closed => write!(f, "closed"),
            PullRequestStatus::Merged => write!(f, "merged"),
            PullRequestStatus::Draft => write!(f, "draft"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Blocked,
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketStatus::Open => write!(f, "open"),
            TicketStatus::InProgress => write!(f, "in-progress"),
            TicketStatus::Resolved => write!(f, "resolved"),
            TicketStatus::Closed => write!(f, "closed"),
            TicketStatus::Blocked => write!(f, "blocked"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketType {
    Bug,
    Feature,
    Task,
    Epic,
    Story,
    Issue,
}

impl std::fmt::Display for TicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TicketType::Bug => write!(f, "bug"),
            TicketType::Feature => write!(f, "feature"),
            TicketType::Task => write!(f, "task"),
            TicketType::Epic => write!(f, "epic"),
            TicketType::Story => write!(f, "story"),
            TicketType::Issue => write!(f, "issue"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DashboardType {
    Kibana,
    Grafana,
    Custom,
}

impl std::fmt::Display for DashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DashboardType::Kibana => write!(f, "kibana"),
            DashboardType::Grafana => write!(f, "grafana"),
            DashboardType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotebookType {
    Databricks,
    Jupyter,
    Zeppelin,
    Custom,
}

impl std::fmt::Display for NotebookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotebookType::Databricks => write!(f, "databricks"),
            NotebookType::Jupyter => write!(f, "jupyter"),
            NotebookType::Zeppelin => write!(f, "zeppelin"),
            NotebookType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipelineType {
    Jenkins,
    GitHubActions,
    GitLabCI,
    CircleCI,
    Travis,
    Custom,
}

impl std::fmt::Display for PipelineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineType::Jenkins => write!(f, "jenkins"),
            PipelineType::GitHubActions => write!(f, "github-actions"),
            PipelineType::GitLabCI => write!(f, "gitlab-ci"),
            PipelineType::CircleCI => write!(f, "circleci"),
            PipelineType::Travis => write!(f, "travis"),
            PipelineType::Custom => write!(f, "custom"),
        }
    }
}

/// Relationship types to represent connections between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    HasRepository,
    HasBranch,
    HasPR,
    HasTicket,
    HasDashboard,
    HasNotebook,
    HasPipeline,
    SourceFor,
    TargetTo,
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipType::HasRepository => write!(f, "HAS_REPOSITORY"),
            RelationshipType::HasBranch => write!(f, "HAS_BRANCH"),
            RelationshipType::HasPR => write!(f, "HAS_PR"),
            RelationshipType::HasTicket => write!(f, "HAS_TICKET"),
            RelationshipType::HasDashboard => write!(f, "HAS_DASHBOARD"),
            RelationshipType::HasNotebook => write!(f, "HAS_NOTEBOOK"),
            RelationshipType::HasPipeline => write!(f, "HAS_PIPELINE"),
            RelationshipType::SourceFor => write!(f, "SOURCE_FOR"),
            RelationshipType::TargetTo => write!(f, "TARGET_TO"),
        }
    }
}
