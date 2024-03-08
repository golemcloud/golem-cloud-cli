// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use clap::Subcommand;
use golem_gateway_client::model::{ApiDeployment, ApiSite};

use crate::clients::gateway::deployment::DeploymentClient;
use crate::clients::project::ProjectClient;
use crate::model::{GolemError, GolemResult, ProjectRef};

#[derive(Subcommand, Debug)]
#[command()]
pub enum DeploymentSubcommand {
    #[command()]
    Get {
        #[command(flatten)]
        project_ref: ProjectRef,
        #[arg(short, long, value_name = "api-definition-id", value_hint = clap::ValueHint::Other)]
        definition_id: String,
    },
    #[command()]
    Add {
        #[command(flatten)]
        project_ref: ProjectRef,
        #[arg(short, long, value_name = "api-definition-id", value_hint = clap::ValueHint::Other)]
        definition_id: String,
        #[arg(short = 'H', long, value_name = "site-host", value_hint = clap::ValueHint::Other)]
        host: String,
        #[arg(short, long, value_name = "site-subdomain", value_hint = clap::ValueHint::Other)]
        subdomain: String,
    },
    #[command()]
    Delete {
        #[command(flatten)]
        project_ref: ProjectRef,
        #[arg(short, long)]
        site: String,
        #[arg(short, long, value_name = "api-definition-id", value_hint = clap::ValueHint::Other)]
        definition_id: String,
    },
}

#[async_trait]
pub trait DeploymentHandler {
    async fn handle(&self, command: DeploymentSubcommand) -> Result<GolemResult, GolemError>;
}

pub struct DeploymentHandlerLive<
    'p,
    C: DeploymentClient + Sync + Send,
    P: ProjectClient + Sync + Send,
> {
    pub client: C,
    pub projects: &'p P,
}

#[async_trait]
impl<'p, C: DeploymentClient + Sync + Send, P: ProjectClient + Sync + Send> DeploymentHandler
    for DeploymentHandlerLive<'p, C, P>
{
    async fn handle(&self, command: DeploymentSubcommand) -> Result<GolemResult, GolemError> {
        match command {
            DeploymentSubcommand::Get {
                project_ref,
                definition_id,
            } => {
                let project_id = self.projects.resolve_id_or_default(project_ref).await?;
                let res = self.client.get(project_id, &definition_id).await?;

                Ok(GolemResult::Ok(Box::new(res)))
            }
            DeploymentSubcommand::Add {
                project_ref,
                definition_id,
                host,
                subdomain,
            } => {
                let deployment = ApiDeployment {
                    project_id: self.projects.resolve_id_or_default(project_ref).await?.0,
                    api_definition_id: definition_id,
                    site: ApiSite { host, subdomain },
                };

                let res = self.client.update(deployment).await?;

                Ok(GolemResult::Ok(Box::new(res)))
            }
            DeploymentSubcommand::Delete {
                project_ref,
                site,
                definition_id,
            } => {
                let project_id = self.projects.resolve_id_or_default(project_ref).await?;
                let res = self
                    .client
                    .delete(project_id, &definition_id, &site)
                    .await?;
                Ok(GolemResult::Ok(Box::new(res)))
            }
        }
    }
}
