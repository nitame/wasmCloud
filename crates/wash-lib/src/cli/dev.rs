use anyhow::Result;
use console::style;
use wasmcloud_control_interface::Client;

use crate::{
    build::{build_project, SignConfig},
    component::update_component,
    generate::emoji,
    id::{ModuleId, ServerId},
    parser::{ProjectConfig, TypeConfig},
};

/// Perform a single execution of the dev loop for an artifact
pub async fn run_dev_loop(
    project_cfg: &ProjectConfig,
    component_id: ModuleId,
    component_ref: &str,
    host_id: ServerId,
    ctl_client: &Client,
    sign_cfg: Option<SignConfig>,
) -> Result<()> {
    let built_artifact_path = build_project(project_cfg, sign_cfg.as_ref())
        .await?
        .canonicalize()?;

    // Restart the artifact so that changes can be observed
    match project_cfg.project_type {
        TypeConfig::Provider(_) => {
            eprintln!(
                "{} {}",
                emoji::WARN,
                style("`wash build` providers are not yet supported for dev, skipping...").bold(),
            );
        }
        TypeConfig::Component(_) => {
            eprintln!(
                "{} {}",
                emoji::RECYCLE,
                style(format!(
                    "restarting component @ [{}]...",
                    built_artifact_path.display()
                ))
                .bold(),
            );

            update_component(ctl_client, &host_id, &component_id, component_ref).await?;
        }
    }

    Ok(())
}
