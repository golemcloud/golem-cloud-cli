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

use crate::model::{AccountId, ProjectAction};
use golem_cloud_client::model::TokenSecret;
use golem_cloud_client::model::UnsafeToken;

pub mod account;
pub mod errors;
pub mod gateway;
pub mod grant;
pub mod login;
pub mod policy;
pub mod project;
pub mod project_grant;
pub mod template;
pub mod token;
pub mod worker;

pub fn token_header(secret: &TokenSecret) -> String {
    format!("bearer {}", secret.value)
}

#[derive(Clone, PartialEq, Debug)]
pub struct CloudAuthentication(pub UnsafeToken);

impl CloudAuthentication {
    pub fn header(&self) -> String {
        let CloudAuthentication(value) = self;

        token_header(&value.secret)
    }

    pub fn account_id(&self) -> AccountId {
        let CloudAuthentication(value) = self;

        AccountId {
            id: value.data.account_id.clone(),
        }
    }
}

pub fn action_cli_to_api(action: ProjectAction) -> golem_cloud_client::model::ProjectAction {
    match action {
        ProjectAction::ViewTemplate => golem_cloud_client::model::ProjectAction::ViewTemplate {},
        ProjectAction::CreateTemplate => {
            golem_cloud_client::model::ProjectAction::CreateTemplate {}
        }
        ProjectAction::UpdateTemplate => {
            golem_cloud_client::model::ProjectAction::UpdateTemplate {}
        }
        ProjectAction::DeleteTemplate => {
            golem_cloud_client::model::ProjectAction::DeleteTemplate {}
        }
        ProjectAction::ViewWorker => golem_cloud_client::model::ProjectAction::ViewWorker {},
        ProjectAction::CreateWorker => golem_cloud_client::model::ProjectAction::CreateWorker {},
        ProjectAction::UpdateWorker => golem_cloud_client::model::ProjectAction::UpdateWorker {},
        ProjectAction::DeleteWorker => golem_cloud_client::model::ProjectAction::DeleteWorker {},
        ProjectAction::ViewProjectGrants => {
            golem_cloud_client::model::ProjectAction::ViewProjectGrants {}
        }
        ProjectAction::CreateProjectGrants => {
            golem_cloud_client::model::ProjectAction::CreateProjectGrants {}
        }
        ProjectAction::DeleteProjectGrants => {
            golem_cloud_client::model::ProjectAction::DeleteProjectGrants {}
        }
    }
}
