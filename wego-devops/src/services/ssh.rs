//! ssh执行脚本

use std::time::Duration;

use tokio::{process::Command, time::timeout};

use crate::settings;

pub async fn run_shell(shell_command: &str) -> anyhow::Result<()> {
    timeout(Duration::from_secs(settings::get_shell_timeout()), async {
        Command::new(settings::get_shell_env())
            .arg("-c")
            .arg(&shell_command)
            .status()            
            .await
    })
    .await??;

    tracing::info!("run shell: {}", shell_command);
    Ok(())
}

/// 执行本地脚本
pub async fn run_local_shell(
    script_content: &str,
    log_file: &str,
    dir: &str,
) -> anyhow::Result<()> {
    let shell_command = format!(
        r#"
cd {}
bash -xe <<EOF 2>&1 | tee -a {}
PS4='+[$(date "+%Y-%m-%d %H:%M:%S")]'
{}
EOF
"#,
        dir, log_file, script_content
    );
    run_shell(&shell_command).await?;
    Ok(())
}

/// 执行远程脚本
pub async fn run_remote_shell(
    host: &str,
    script_content: &str,
    log_file: &str,
) -> anyhow::Result<()> {
    let content = script_content.replace("$", "\\$").replace(r#"""#, r#"\""#);
    let shell_command = format!(
        r#"
ssh -t {} "
set -xe 
export PS4='+[$(date "+%Y-%m-%d %H:%M:%S")]'
{}
" | tee -a {}
        "#,
        host, content, log_file
    );
    run_shell(&shell_command).await?;
    Ok(())
}
