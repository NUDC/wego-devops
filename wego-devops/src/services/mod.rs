use serde::{Deserialize, Serialize};

pub mod group;
pub mod project;
pub mod server;
mod ssh;

pub use group::{ProjectIndex, ProjectUniqueId};
pub use project::ProjectConfig;
pub use server::Server;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub name: String,
    pub remark: String,
    pub build_script: String,
    pub deploy_script: String,
}
