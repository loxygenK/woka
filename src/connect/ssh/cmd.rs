use std::{
    io::{BufRead as _, BufReader},
    process::{Command, ExitStatus, Stdio},
};

use crate::connect::app::{ConnectOptions, PortForward};

pub struct SSHCommand(Command);

impl SSHCommand {
    pub fn new(hostname: &str, option: &ConnectOptions) -> Self {
        let mut cmd = Command::new("ssh");
        cmd.arg("-o")
            .arg("ConnectTimeout=10")
            .arg("-o")
            .arg("BatchMode=no")
            .arg("-t");

        for port_forward in &option.port_forwards {
            let port_forward_str = format!(
                "{}:localhost:{}",
                port_forward.local_port(),
                port_forward.remote_port()
            );

            match port_forward {
                PortForward::Local(_, _) => cmd.arg("-L").arg(port_forward_str),
                PortForward::Remote(_, _) => cmd.arg("-R").arg(port_forward_str),
            };
        }

        cmd.arg(hostname);

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::piped());

        if !option.cmds.is_empty() {
            if option.interactive_shell {
                // TODO: use `shell-escape` crate for this
                // let escaped = option
                //     .cmds
                //     .join(" ")
                //     .replace(r"\", r"\\")
                //     .replace(r"'", r"\'");

                cmd.arg("exec")
                    .arg("$SHELL")
                    .arg("-li")
                    .arg("-c")
                    // .arg(format!("'{escaped}'"));
                    .arg(format!("'{}'", option.cmds.join(" ")));
            } else {
                cmd.args(&option.cmds);
            }
        }

        Self(cmd)
    }

    pub fn connect(&mut self) -> Result<ExitStatus, SSHCommandError> {
        let Ok(mut child) = self.0.spawn() else {
            return Err(SSHCommandError::ExecutionFailed);
        };

        let Some(stderr) = child.stderr.take() else {
            child.kill().ok();
            panic!("Expected 'stderr' to be available but it is not");
        };

        let mut stderr = BufReader::new(stderr);

        let mut latest_msg = String::new();
        let mut buf = String::new();
        while let Ok(size) = stderr.read_line(&mut buf) {
            if size == 0 {
                // EOF
                break;
            }
            eprint!("\x1b[38;5;248m{buf}\x1b[m");

            latest_msg = buf;
            buf = String::new();
        }

        let Ok(output) = child.wait_with_output() else {
            return Err(SSHCommandError::ExecutionFailed);
        };

        // XXX: I really need to figure out more reliable way
        if latest_msg.starts_with("ssh: ") {
            Err(SSHCommandError::ConnectionFailed)
        } else {
            Ok(output.status)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SSHCommandError {
    #[error("Could not execute ssh command - is ssh installed?")]
    ExecutionFailed,

    #[error("Could not connect to the host")]
    ConnectionFailed,
}
