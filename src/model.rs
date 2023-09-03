use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;
use clap::{ArgMatches, Error, FromArgMatches};
use derive_more::{Display, FromStr, Into};
use golem_client::account::AccountError;
use golem_client::component::ComponentError;
use golem_client::grant::GrantError;
use golem_client::login::LoginError;
use golem_client::project::ProjectError;
use golem_client::project_grant::ProjectGrantError;
use golem_client::project_policy::ProjectPolicyError;
use golem_client::token::TokenError;
use indoc::indoc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum GolemResult {
    Ok(Box<dyn PrintRes>),
    Str(String),
}

impl GolemResult {
    pub fn err(s: String) -> Result<GolemResult, GolemError> {
        Err(GolemError(s))
    }
}

pub trait PrintRes {
    fn println(&self, format: &Format) -> ();
}

impl<T> PrintRes for T
    where T: Serialize, {
    fn println(&self, format: &Format) -> () {
        match format {
            Format::Json => println!("{}", serde_json::to_string_pretty(self).unwrap()),
            Format::Yaml => println!("{}", serde_yaml::to_string(self).unwrap()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct GolemError(pub String);


impl From<AccountError> for GolemError {
    fn from(value: AccountError) -> Self {
        match value {
            AccountError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            AccountError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            AccountError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            AccountError::Status404 { message } => GolemError(format!("Not found: {message}")),
            AccountError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            }
            AccountError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl From<TokenError> for GolemError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            TokenError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            TokenError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            TokenError::Status404 { message } => GolemError(format!("Not found: {message}")),
            TokenError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            }
            TokenError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl From<ComponentError> for GolemError {
    fn from(value: ComponentError) -> Self {
        match value {
            ComponentError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            ComponentError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            ComponentError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            ComponentError::Status504 => GolemError(format!("Gateway Timeout")),
            ComponentError::Status404 { message } => GolemError(message),
            ComponentError::Status403 { error } => GolemError(format!("Limit Exceeded: {error}")),
            ComponentError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            },
            ComponentError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
            ComponentError::Status409 { component_id } => GolemError(format!("{component_id} already exists")),
        }
    }
}

impl From<LoginError> for GolemError {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            LoginError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            LoginError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            LoginError::Status403 { .. } => {
                let msg = indoc! {"
                    At the moment account creation is restricted.
                    None of your verified emails is whitelisted.
                    Please contact us to create an account.
                "};
                GolemError(msg.to_string())
            }
            LoginError::Status500 { error } => GolemError(format!("Internal server error on Login: {error}")),
            LoginError::Status401 { error } => GolemError(format!("External service call error on Login: {error}")),
        }
    }
}

impl From<ProjectError> for GolemError {
    fn from(value: ProjectError) -> Self {
        match value {
            ProjectError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            ProjectError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            ProjectError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            ProjectError::Status404 { message } => GolemError(format!("Not found: {message}")),
            ProjectError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            }
            ProjectError::Status403 { error } => GolemError(format!("Limit Exceeded: {error}")),
            ProjectError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl From<GrantError> for GolemError {
    fn from(value: GrantError) -> Self {
        match value {
            GrantError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            GrantError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            GrantError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            GrantError::Status404 { message } => GolemError(format!("Not found: {message}")),
            GrantError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            },
            GrantError::Status500 { error } =>  GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl From<ProjectPolicyError> for GolemError {
    fn from(value: ProjectPolicyError) -> Self {
        match value {
            ProjectPolicyError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            ProjectPolicyError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            ProjectPolicyError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            ProjectPolicyError::Status404 { message } => GolemError(format!("Not found: {message}")),
            ProjectPolicyError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            } ,
            ProjectPolicyError::Status403 { error } => GolemError(format!("Limit Exceeded: {error}")),
            ProjectPolicyError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl From<ProjectGrantError> for GolemError {
    fn from(value: ProjectGrantError) -> Self {
        match value {
            ProjectGrantError::RequestFailure(err) => GolemError(format!("Unexpected request failure: {err}")),
            ProjectGrantError::InvalidHeaderValue(err) =>  GolemError(format!("Unexpected invalid header value: {err}")),
            ProjectGrantError::UnexpectedStatus(sc) =>  GolemError(format!("Unexpected status: {sc}")),
            ProjectGrantError::Status404 { message } => GolemError(format!("Not found: {message}")),
            ProjectGrantError::Status400 { errors } => {
                let msg = errors.join(", ");
                GolemError(format!("Invalid API call: {msg}"))
            } ,
            ProjectGrantError::Status403 { error } => GolemError(format!("Limit Exceeded: {error}")),
            ProjectGrantError::Status500 { error } => GolemError(format!("Internal server error: {error}")),
        }
    }
}

impl Display for GolemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let GolemError(s) = self;
        Display::fmt(s, f)
    }
}

impl Debug for GolemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let GolemError(s) = self;
        Display::fmt(s, f)
    }
}

impl std::error::Error for GolemError {
    fn description(&self) -> &str {
        let GolemError(s) = self;

        s
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter)]
pub enum Format {
    Json,
    Yaml,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Json => "json",
            Self::Yaml => "yaml",
        };
        Display::fmt(&s, f)
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => {
                let all =
                    Format::iter()
                        .map(|x| format!("\"{x}\""))
                        .collect::<Vec<String>>()
                        .join(", ");
                Err(format!("Unknown format: {s}. Expected one of {all}"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, FromStr)]
pub struct AccountId {
    pub id: String,
} // TODO: Validate


impl AccountId {
    pub fn new(id: String) -> AccountId {
        AccountId { id }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, FromStr, Into)]
pub struct TokenId(pub Uuid);

#[derive(Clone, PartialEq, Eq, Debug, Into)]
pub struct ProjectId(pub Uuid);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ProjectRef {
    Id(ProjectId),
    Name(String),
    Default,
}

impl FromArgMatches for ProjectRef {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        ProjectRefArgs::from_arg_matches(matches).map(|c| (&c).into())
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let prc0: ProjectRefArgs = (&self.clone()).into();
        let mut prc = prc0.clone();
        let res = ProjectRefArgs::update_from_arg_matches(&mut prc, matches);
        *self = (&prc).into();
        res
    }
}

impl clap::Args for ProjectRef {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        ProjectRefArgs::augment_args(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        ProjectRefArgs::augment_args_for_update(cmd)
    }
}

#[derive(clap::Args, Debug, Clone)]
struct ProjectRefArgs {
    #[arg(short = 'P', long, conflicts_with = "project_name")]
    project_id: Option<Uuid>,

    #[arg(short = 'p', long, conflicts_with = "project_id")]
    project_name: Option<String>,
}

impl From<&ProjectRefArgs> for ProjectRef {
    fn from(value: &ProjectRefArgs) -> ProjectRef {
        if let Some(id) = value.project_id {
            ProjectRef::Id(ProjectId(id))
        } else if let Some(name) = value.project_name.clone() {
            ProjectRef::Name(name)
        } else {
            ProjectRef::Default
        }
    }
}

impl From<&ProjectRef> for ProjectRefArgs {
    fn from(value: &ProjectRef) -> Self {
        match value {
            ProjectRef::Id(ProjectId(id)) => {
                ProjectRefArgs { project_id: Some(id.clone()), project_name: None }
            }
            ProjectRef::Name(name) => {
                ProjectRefArgs { project_id: None, project_name: Some(name.clone()) }
            }
            ProjectRef::Default => {
                ProjectRefArgs { project_id: None, project_name: None }
            }
        }
    }
}

impl FromArgMatches for ComponentIdOrName {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        ComponentIdOrNameArgs::from_arg_matches(matches).map(|c| (&c).into())
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let prc0: ComponentIdOrNameArgs = (&self.clone()).into();
        let mut prc = prc0.clone();
        let res = ComponentIdOrNameArgs::update_from_arg_matches(&mut prc, matches);
        *self = (&prc).into();
        res
    }
}

impl clap::Args for ComponentIdOrName {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        ComponentIdOrNameArgs::augment_args(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        ComponentIdOrNameArgs::augment_args_for_update(cmd)
    }
}

#[derive(clap::Args, Debug, Clone)]
struct ComponentIdOrNameArgs {
    #[arg(short = 'C', long, conflicts_with = "component_name", required = true)]
    component_id: Option<Uuid>,

    #[arg(short, long, conflicts_with = "component_id", required = true)]
    component_name: Option<String>,

    #[arg(short = 'P', long, conflicts_with = "project_name", conflicts_with = "component_id")]
    project_id: Option<Uuid>,

    #[arg(short = 'p', long, conflicts_with = "project_id", conflicts_with = "component_id")]
    project_name: Option<String>,
}


impl From<&ComponentIdOrNameArgs> for ComponentIdOrName {
    fn from(value: &ComponentIdOrNameArgs) -> ComponentIdOrName {
        let pr = if let Some(id) = value.project_id {
            ProjectRef::Id(ProjectId(id))
        } else if let Some(name) = value.project_name.clone() {
            ProjectRef::Name(name)
        } else {
            ProjectRef::Default
        };

        if let Some(id) = value.component_id {
            ComponentIdOrName::Id(RawComponentId(id))
        } else {
            ComponentIdOrName::Name(ComponentName(value.component_name.as_ref().unwrap().to_string()), pr)
        }
    }
}

impl From<&ComponentIdOrName> for ComponentIdOrNameArgs {
    fn from(value: &ComponentIdOrName) -> ComponentIdOrNameArgs {
        match value {
            ComponentIdOrName::Id(RawComponentId(id)) => {
                ComponentIdOrNameArgs { component_id: Some(id.clone()), component_name: None, project_id: None, project_name: None }
            }
            ComponentIdOrName::Name(ComponentName(name), pr) => {
                let (project_id, project_name) = match pr {
                    ProjectRef::Id(ProjectId(id)) => {
                        (Some(*id), None)
                    }
                    ProjectRef::Name(name) => {
                        (None, Some(name.to_string()))
                    }
                    ProjectRef::Default => {
                        (None, None)
                    }
                };

                ComponentIdOrNameArgs { component_id: None, component_name: Some(name.clone()), project_id, project_name }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawComponentId(pub Uuid);

#[derive(Clone, PartialEq, Eq, Debug, Display, FromStr)]
pub struct ComponentName(pub String); // TODO: Validate

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ComponentIdOrName {
    Id(RawComponentId),
    Name(ComponentName, ProjectRef),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter, Serialize, Deserialize)]
pub enum Role {
    Admin,
    WhitelistAdmin,
    MarketingAdmin,
    ViewProject,
    DeleteProject,
    CreateProject,
    InstanceServer,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Role::Admin => { "Admin" }
            Role::WhitelistAdmin => { "WhitelistAdmin" }
            Role::MarketingAdmin => { "MarketingAdmin" }
            Role::ViewProject => { "ViewProject" }
            Role::DeleteProject => { "DeleteProject" }
            Role::CreateProject => { "CreateProject" }
            Role::InstanceServer => { "InstanceServer" }
        };

        Display::fmt(s, f)
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Role::Admin),
            "WhitelistAdmin" => Ok(Role::WhitelistAdmin),
            "MarketingAdmin" => Ok(Role::MarketingAdmin),
            "ViewProject" => Ok(Role::ViewProject),
            "DeleteProject" => Ok(Role::DeleteProject),
            "CreateProject" => Ok(Role::CreateProject),
            "InstanceServer" => Ok(Role::InstanceServer),
            _ => {
                let all =
                    Role::iter()
                        .map(|x| format!("\"{x}\""))
                        .collect::<Vec<String>>()
                        .join(", ");
                Err(format!("Unknown role: {s}. Expected one of {all}"))
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter)]
pub enum ProjectAction {
    ViewComponent,
    CreateComponent,
    UpdateComponent,
    DeleteComponent,
    ViewInstance,
    CreateInstance,
    UpdateInstance,
    DeleteInstance,
    ViewProjectGrants,
    CreateProjectGrants,
    DeleteProjectGrants,
}

impl Display for ProjectAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProjectAction::ViewComponent => "ViewComponent",
            ProjectAction::CreateComponent => "CreateComponent",
            ProjectAction::UpdateComponent => "UpdateComponent",
            ProjectAction::DeleteComponent => "DeleteComponent",
            ProjectAction::ViewInstance => "ViewInstance",
            ProjectAction::CreateInstance => "CreateInstance",
            ProjectAction::UpdateInstance => "UpdateInstance",
            ProjectAction::DeleteInstance => "DeleteInstance",
            ProjectAction::ViewProjectGrants => "ViewProjectGrants",
            ProjectAction::CreateProjectGrants => "CreateProjectGrants",
            ProjectAction::DeleteProjectGrants => "DeleteProjectGrants",
        };

        Display::fmt(s, f)
    }
}

impl FromStr for ProjectAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ViewComponent" => Ok(ProjectAction::ViewComponent),
            "CreateComponent" => Ok(ProjectAction::CreateComponent),
            "UpdateComponent" => Ok(ProjectAction::UpdateComponent),
            "DeleteComponent" => Ok(ProjectAction::DeleteComponent),
            "ViewInstance" => Ok(ProjectAction::ViewInstance),
            "CreateInstance" => Ok(ProjectAction::CreateInstance),
            "UpdateInstance" => Ok(ProjectAction::UpdateInstance),
            "DeleteInstance" => Ok(ProjectAction::DeleteInstance),
            "ViewProjectGrants" => Ok(ProjectAction::ViewProjectGrants),
            "CreateProjectGrants" => Ok(ProjectAction::CreateProjectGrants),
            "DeleteProjectGrants" => Ok(ProjectAction::DeleteProjectGrants),
            _ => {
                let all =
                    ProjectAction::iter()
                        .map(|x| format!("\"{x}\""))
                        .collect::<Vec<String>>()
                        .join(", ");
                Err(format!("Unknown action: {s}. Expected one of {all}"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, FromStr)]
pub struct ProjectPolicyId(pub Uuid);