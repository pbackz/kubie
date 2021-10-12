use std::io::{BufWriter, Write};
use std::process::Command;

use anyhow::Result;

use super::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    let temp_rc_file = tempfile::Builder::new()
        .prefix("kubie-tcshrc")
        .suffix(".tcsh")
        .tempfile()?;
    let mut temp_rc_file_buf = BufWriter::new(temp_rc_file.as_file());

    write!(
        temp_rc_file_buf,
        r#"
if [[ "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    if [[ -f /usr/site/etc/system.cshrc ]] ; then
        source "/usr/site/etc/system.cshrc"
    fi
else
    source "$HOME/.cshrc"
fi

function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

trap '__kubie_cmd_pre_exec__' DEBUG
"#
    )?;

    temp_rc_file_buf.flush()?;

    let mut cmd = Command::new("tcsh");
    cmd.arg("--rcfile");
    cmd.arg(temp_rc_file.path());
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
