use std::process::Command;

use crate::connect::app::PortForward;

pub fn construct_ssh_cmd(
    hostname: &str,
    port_forwards: &[PortForward],
) -> Command {
    let mut cmd = Command::new("ssh");
    cmd.arg("-o").arg("ConnectTimeout=10")
       .arg("-o").arg("BatchMode=no");

    for port_forward in port_forwards {
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

    cmd
}

